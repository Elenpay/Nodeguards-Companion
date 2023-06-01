use anyhow::{anyhow, Context, Result};
use bitcoin::consensus::serialize;
use bitcoin::psbt::Input;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::secp256k1::{All, Message, Secp256k1};
use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::util::psbt::PartiallySignedTransaction;
use bitcoin::util::sighash::SighashCache;
use bitcoin::{EcdsaSig, EcdsaSighashType, Network, PublicKey};
use std::str::FromStr;

use crate::utils::base64::to_base64;
use crate::wallet::Wallet;

fn set_sighash_type(signature: Signature, input: &Input) -> EcdsaSig {
    let sighash_type = get_sighash_type(input);
    EcdsaSig {
        sig: signature,
        hash_ty: sighash_type,
    }
}

fn get_sighash_type(input: &Input) -> EcdsaSighashType {
    input
        .sighash_type
        .and_then(|t| t.ecdsa_hash_ty().ok())
        .unwrap_or(EcdsaSighashType::All)
}

fn get_partial_derivation(
    derivation: &DerivationPath,
    sub_derivation: &DerivationPath,
) -> Result<DerivationPath> {
    if derivation.len() > sub_derivation.len() {
        return Err(anyhow!(
            "Can't get a partial derivation from a derivation greater than the sub derivation"
        ));
    }
    let partial = &sub_derivation[derivation.len()..];
    DerivationPath::try_from(partial).map_err(|e| anyhow!("{e}"))
}

fn derive_relative_xpriv(
    xprv: &ExtendedPrivKey,
    secp: &Secp256k1<All>,
    derivation: &DerivationPath,
    sub_derivation: &DerivationPath,
) -> Result<ExtendedPrivKey> {
    xprv.derive_priv(secp, &get_partial_derivation(derivation, sub_derivation)?)
        .map_err(|e| anyhow!("{e}"))
}

fn sign_psbt(
    mut psbt: PartiallySignedTransaction,
    xprv: ExtendedPrivKey,
    derivation: &DerivationPath,
) -> Result<PartiallySignedTransaction> {
    let secp = Secp256k1::new();

    // https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki#user-content-Signer

    for (index, input) in psbt.inputs.iter_mut().enumerate() {
        let witness_script = input
            .witness_script
            .as_ref()
            .context("Missing witness script")?;

        let amount = input
            .witness_utxo
            .as_ref()
            .context("Witness utxo not found")?
            .value;

        let mut sighash_cache = SighashCache::new(&psbt.unsigned_tx);
        let sighash = sighash_cache.segwit_signature_hash(
            index,
            witness_script,
            amount,
            get_sighash_type(input),
        )?;

        let mut input_keypairs = Vec::new();

        for (_, (fingerprint, sub_derivation)) in input.bip32_derivation.iter() {
            if fingerprint != &xprv.fingerprint(&secp) {
                continue;
            }
            let parent_xprv = derive_relative_xpriv(&xprv, &secp, derivation, sub_derivation)?;
            input_keypairs.push(parent_xprv.to_keypair(&secp));
        }

        if input_keypairs.is_empty() {
            return Err(anyhow!("No private keys to sign this psbt"));
        }

        for keypair in input_keypairs {
            let message = &Message::from_slice(&sighash)?;
            let signature = secp.sign_ecdsa(message, &keypair.secret_key());
            input.partial_sigs.insert(
                PublicKey::new(keypair.public_key()),
                set_sighash_type(signature, input),
            );

            secp.verify_ecdsa(message, &signature, &keypair.public_key())?;
        }
    }

    Ok(psbt)
}

pub fn decode_psbt_and_sign(
    psbt_64: &str,
    wallet: &mut Wallet,
    password: &str,
    network: Network,
) -> Result<String> {
    let psbt = PartiallySignedTransaction::from_str(psbt_64)?;

    let xprv = wallet.get_xprv(password, network)?;
    let signed_psbt = sign_psbt(psbt, xprv, &wallet.derivation)?;

    Ok(to_base64(&serialize(&signed_psbt)))
}

#[test]
fn derives_all() {
    let derivation = DerivationPath::from_str("m").unwrap();
    let sub_derivation = DerivationPath::from_str("m/1'/1'/0/0").unwrap();
    let partial_derivation = DerivationPath::from_str("m/1'/1'/0/0").unwrap();
    let result = get_partial_derivation(&derivation, &sub_derivation).unwrap();
    assert_eq!(result, partial_derivation);
}

#[test]
fn derives_partial() {
    let derivation = DerivationPath::from_str("m/1'/1'").unwrap();
    let sub_derivation = DerivationPath::from_str("m/1'/1'/0/0").unwrap();
    let partial_derivation = DerivationPath::from_str("m/0/0").unwrap();
    let result = get_partial_derivation(&derivation, &sub_derivation).unwrap();
    assert_eq!(result, partial_derivation);
}

#[test]
fn sign() {
    let mut wallet = Wallet::default();
    wallet.from_seed_str("wallet 1", "social mango annual basic work brain economy one safe physical junk other toy valid load cook napkin maple runway island oil fan legend stem", "Qwerty123").unwrap();

    // wallet.from_xprv_str("wallet 1", "tprv8ZgxMBicQKsPduvXYAnkop1b1UoAY2pS68pe9jHuJwuMvx6G5sh4C67peYZkRawdBWbMbfoybgQJ3g8nTZAezEeHyaW9A9UjtpTRmSyJwUn", "m/48'/1'/1'", "Qwerty123").unwrap();
    let signed_psbt = decode_psbt_and_sign(
        "cHNidP8BAIcBAAAAAvTkJTkzr4vYO7aoN4AUZaL3BkMWYQ+mYPFBtjhVmvCSAAAAAAD/////glBxTfjYVipVX+gO+YUDCVSJdWtsFo7aTRQ25n0zqwMAAAAAAP////8BbcQAAAAAAAAiACCNY6b6UCml4qW1ojbPwqJ17aaifLtEGzgCwA97J8cCuAAAAABPAQQ1h88DLxEkLIAAAAH8O+5COgGM620iagZjKI7Q+17g+p15CrNw0DDgRzDmyAOAji4c1IK90HGWlDxagTPRaa+GAitrsDgXcWROwHRE1xAfzOTeMAAAgAEAAIABAACATwEENYfPA1asA/SAAAAB+QiuSmaw/6GrFLVa8M5I8qcCJcuQEJAtaQGvkLk+j4ECUa8V4Fu4t+rBiVs/X0f/7+zYMhoC7yIpj4nQogN98GAQYPOgszAAAIABAACAAQAAgE8BBDWHzwN9uUaNgAAAAfUicMdoejr7NR708bmjDBDzttOSFwaDrqlIjBOaEcmYAyIjhSM46mf7/Y3DUqsmJeeaJ3ZGISNr8cAhMJDTSJ6fEO0CEMgwAACAAQAAgAEAAIAAAQErU8MAAAAAAAAiACADQDxIJhQEIWqn08T37W/Ih8uXW40ehgg3slu0Uf7dRQEDBAIAAAABBWlSIQJDnVTFh6pHCQmatEEVvV6sf1P5+u9Bb7zhz0D829Ay2CEDC0VHn9A3/8QMIRBpZobsevmCDpADo8aiHknTJs9b234hA+HpTXYQbhjCPCQRmQlLH4fKO27ghsThI+ztTmujeM2ZU64iBgJDnVTFh6pHCQmatEEVvV6sf1P5+u9Bb7zhz0D829Ay2BjtAhDIMAAAgAEAAIABAACAAAAAAPIAAAAiBgMLRUef0Df/xAwhEGlmhux6+YIOkAOjxqIeSdMmz1vbfhgfzOTeMAAAgAEAAIABAACAAAAAAPIAAAAiBgPh6U12EG4YwjwkEZkJSx+Hyjtu4IbE4SPs7U5ro3jNmRhg86CzMAAAgAEAAIABAACAAAAAAPIAAAAAAQErOgUAAAAAAAAiACBdERsXcaX+/OJlazhLTNrrL9dmXPtIFX3+wl/vS3rlbgEDBAIAAAABBWlSIQI8/6zXULU4/iE7RqJR664w7V/JTpxAxkBflCGOxcM8CiECTbNidmQvwtF0A/gs/Q/H4yNY+Vhw0h7ZSQ+MZk/T0PMhAtEgc7jMqsCndxV78/4aPsQ5PCOZoIBbzLmbkAzh6xQLU64iBgI8/6zXULU4/iE7RqJR664w7V/JTpxAxkBflCGOxcM8ChgfzOTeMAAAgAEAAIABAACAAAAAAPUAAAAiBgJNs2J2ZC/C0XQD+Cz9D8fjI1j5WHDSHtlJD4xmT9PQ8xhg86CzMAAAgAEAAIABAACAAAAAAPUAAAAiBgLRIHO4zKrAp3cVe/P+Gj7EOTwjmaCAW8y5m5AM4esUCxjtAhDIMAAAgAEAAIABAACAAAAAAPUAAAAAAA==",
        &mut wallet,
        "Qwerty123",
        Network::Regtest,
    );
    dbg!("{:?}", &signed_psbt);
    assert!(signed_psbt.is_ok());
}
