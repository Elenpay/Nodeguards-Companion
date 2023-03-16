use std::str::FromStr;
use base64::Engine;
use base64::engine::general_purpose;
use bitcoin::consensus::serialize;
use bitcoin::psbt::Input;
use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::util::sighash::SighashCache;
use bitcoin::{EcdsaSighashType, PublicKey, EcdsaSig};
use bitcoin::secp256k1::{Secp256k1, Message};
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::psbt::PartiallySignedTransaction;
use anyhow::{Context, Result, anyhow};


use crate::wallet::Wallet;

fn set_sighash_type<'a>(signature: Signature, input: &Input) -> EcdsaSig {
    let sighash_type = get_sighash_type(input);
    let mut sig = EcdsaSig::sighash_all(signature);
    sig.hash_ty = sighash_type;
    sig
}

fn get_sighash_type(input: &Input) -> EcdsaSighashType {
    input.sighash_type
        .and_then(|t| t.ecdsa_hash_ty().ok())
        .unwrap_or(EcdsaSighashType::All)
}

fn sign_psbt(mut psbt: PartiallySignedTransaction, xprv: ExtendedPrivKey) -> Result<PartiallySignedTransaction> {
    let secp = Secp256k1::new();

    // https://github.com/bitcoin/bips/blob/master/bip-0174.mediawiki#user-content-Signer

    for (index, input) in psbt.inputs.iter_mut().enumerate() {
        let witness_script = input.witness_script.as_ref().context("Missing witness script")?;

        let mut sighash_cache = SighashCache::new(&psbt.unsigned_tx);
        let amount = input.witness_utxo.as_ref().context("Witness utxo not found")?.value;
        let sighash = sighash_cache.segwit_signature_hash(
            index, 
            &witness_script, 
            amount, 
            get_sighash_type(&input))?;

        let mut input_keypairs = Vec::new();
        
        for (_, (fingerprint, derivation)) in input.bip32_derivation.iter() {
            if fingerprint != &xprv.fingerprint(&secp) {
                continue;
            }
            let parent_xprv = xprv.derive_priv(&secp, &derivation)?;
            input_keypairs.push(parent_xprv.to_keypair(&secp));
        }
        
        if input_keypairs.len() == 0 {
            return Err(anyhow!("No private keys to sign this psbt"));
        }

        for keypair in input_keypairs {
            let message = &Message::from_slice(&sighash)?;
            let signature = secp.sign_ecdsa(message, &keypair.secret_key());
            input.partial_sigs.insert(PublicKey::new(keypair.public_key()), set_sighash_type(signature, &input));

            secp.verify_ecdsa(message, &signature, &keypair.public_key())?;
        }
    }

    Ok(psbt)
}

pub fn decode_psbt_and_sign(psbt_64: &str) {
    // let wallet = Wallet::from_mnemonic_str("social mango annual basic work brain economy one safe physical junk other toy valid load cook napkin maple runway island oil fan legend stem");
    let wallet = Wallet::from_mnemonic_str("wallet 1", "solar goat auto bachelor chronic input twin depth fork scale divorce fury mushroom column image sauce car public artist announce treat spend jacket physical");
    let psbt = PartiallySignedTransaction::from_str(&psbt_64).unwrap();

    let signed_psbt = sign_psbt(psbt, wallet.xprv.unwrap()).unwrap();
    
    println!("{}", general_purpose::STANDARD.encode(&serialize(&signed_psbt)));
}