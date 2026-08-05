//! NGC-01 regression tests — the approval screen must describe what is
//! actually being signed, from the PSBT alone.
//!
//! The original finding: `PSBTDetails` carried only `tx_id` and `fee`, the
//! "Operation Type" and "Amount" fields were scraped from the DOM of the page
//! requesting the signature, an unparseable PSBT rendered as an innocuous
//! empty screen with Sign enabled, and an uncomputable fee rendered as 0.
//!
//! After the fix:
//!   - every destination and amount is derived from the PSBT and rendered
//!     (`PSBTDetails::display_fields`, which is what the UI component draws),
//!     so recipient substitution is visible;
//!   - page-supplied values no longer exist in the data model;
//!   - an undecodable PSBT yields `ApprovalScreen::Refuse`, which does not
//!     allow signing;
//!   - an uncomputable fee is `None` ("unknown"), never 0;
//!   - a crafted PSBT cannot panic the parse and so cannot bypass the refusal.
//!
//! Run: cargo test --test ngc01_blind_confirmation -- --nocapture

use bitcoin::blockdata::script::Builder;
use bitcoin::consensus::serialize;
use bitcoin::util::psbt::PartiallySignedTransaction as Psbt;
use bitcoin::{Address, Transaction, TxOut};
use signer::psbt_details::{ApprovalScreen, PSBTDetails};
use signer::Network;
use std::str::FromStr;

/// The PSBT from signer/src/signer.rs:150. Two inputs, one output, regtest.
const PSBT: &str = "cHNidP8BAIcBAAAAAvTkJTkzr4vYO7aoN4AUZaL3BkMWYQ+mYPFBtjhVmvCSAAAAAAD/////glBxTfjYVipVX+gO+YUDCVSJdWtsFo7aTRQ25n0zqwMAAAAAAP////8BbcQAAAAAAAAiACCNY6b6UCml4qW1ojbPwqJ17aaifLtEGzgCwA97J8cCuAAAAABPAQQ1h88DLxEkLIAAAAH8O+5COgGM620iagZjKI7Q+17g+p15CrNw0DDgRzDmyAOAji4c1IK90HGWlDxagTPRaa+GAitrsDgXcWROwHRE1xAfzOTeMAAAgAEAAIABAACATwEENYfPA1asA/SAAAAB+QiuSmaw/6GrFLVa8M5I8qcCJcuQEJAtaQGvkLk+j4ECUa8V4Fu4t+rBiVs/X0f/7+zYMhoC7yIpj4nQogN98GAQYPOgszAAAIABAACAAQAAgE8BBDWHzwN9uUaNgAAAAfUicMdoejr7NR708bmjDBDzttOSFwaDrqlIjBOaEcmYAyIjhSM46mf7/Y3DUqsmJeeaJ3ZGISNr8cAhMJDTSJ6fEO0CEMgwAACAAQAAgAEAAIAAAQErU8MAAAAAAAAiACADQDxIJhQEIWqn08T37W/Ih8uXW40ehgg3slu0Uf7dRQEDBAIAAAABBWlSIQJDnVTFh6pHCQmatEEVvV6sf1P5+u9Bb7zhz0D829Ay2CEDC0VHn9A3/8QMIRBpZobsevmCDpADo8aiHknTJs9b234hA+HpTXYQbhjCPCQRmQlLH4fKO27ghsThI+ztTmujeM2ZU64iBgJDnVTFh6pHCQmatEEVvV6sf1P5+u9Bb7zhz0D829Ay2BjtAhDIMAAAgAEAAIABAACAAAAAAPIAAAAiBgMLRUef0Df/xAwhEGlmhux6+YIOkAOjxqIeSdMmz1vbfhgfzOTeMAAAgAEAAIABAACAAAAAAPIAAAAiBgPh6U12EG4YwjwkEZkJSx+Hyjtu4IbE4SPs7U5ro3jNmRhg86CzMAAAgAEAAIABAACAAAAAAPIAAAAAAQErOgUAAAAAAAAiACBdERsXcaX+/OJlazhLTNrrL9dmXPtIFX3+wl/vS3rlbgEDBAIAAAABBWlSIQI8/6zXULU4/iE7RqJR664w7V/JTpxAxkBflCGOxcM8CiECTbNidmQvwtF0A/gs/Q/H4yNY+Vhw0h7ZSQ+MZk/T0PMhAtEgc7jMqsCndxV78/4aPsQ5PCOZoIBbzLmbkAzh6xQLU64iBgI8/6zXULU4/iE7RqJR664w7V/JTpxAxkBflCGOxcM8ChgfzOTeMAAAgAEAAIABAACAAAAAAPUAAAAiBgJNs2J2ZC/C0XQD+Cz9D8fjI1j5WHDSHtlJD4xmT9PQ8xhg86CzMAAAgAEAAIABAACAAAAAAPUAAAAiBgLRIHO4zKrAp3cVe/P+Gj7EOTwjmaCAW8y5m5AM4esUCxjtAhDIMAAAgAEAAIABAACAAAAAAPUAAAAAAA==";

const NETWORK: Network = Network::Regtest;

fn to_base64(psbt: &Psbt) -> String {
    use base64::{engine::general_purpose, Engine};
    general_purpose::STANDARD.encode(serialize(psbt))
}

fn parse(psbt_64: &str) -> PSBTDetails {
    PSBTDetails::parse(psbt_64, NETWORK).expect("PSBT should decode")
}

/// The rows the approval screen actually renders — this is the production
/// formatter the Yew component draws (components/psbt_summary.rs), not a
/// test-local copy of it.
fn rendered_approval_screen(details: &PSBTDetails) -> Vec<(String, String)> {
    details.display_fields()
}

fn field<'a>(screen: &'a [(String, String)], label: &str) -> &'a str {
    &screen
        .iter()
        .find(|(k, _)| k == label)
        .unwrap_or_else(|| panic!("{label} is not on the approval screen"))
        .1
}

#[test]
fn t1_every_destination_and_amount_in_the_psbt_reaches_the_user() {
    let psbt = Psbt::from_str(PSBT).unwrap();
    let details = parse(PSBT);
    let screen = rendered_approval_screen(&details);

    assert_eq!(details.outputs.len(), psbt.unsigned_tx.output.len());
    for (index, output) in psbt.unsigned_tx.output.iter().enumerate() {
        let expected_address = Address::from_script(&output.script_pubkey, NETWORK)
            .unwrap()
            .to_string();
        let row = field(&screen, &format!("Output {}", index + 1));
        assert!(row.contains(&expected_address), "destination is shown");
        assert!(
            row.contains(&output.value.to_string()),
            "per-output amount is shown"
        );
        println!("rendered: {row}");
    }

    assert_eq!(details.total_output(), Some(50_285));
    assert_eq!(field(&screen, "Total"), "50285 SATS");
    assert_eq!(field(&screen, "Fee"), "1056 SATS");
    // The network the addresses were rendered for is stated, so a user on a
    // testnet deployment is not silently shown mainnet-looking addresses.
    assert_eq!(field(&screen, "Network"), NETWORK.to_string());
    println!("-> destination, per-output amount, total, fee and network all derive from the PSBT");
}

#[test]
fn t2_two_transactions_with_different_recipients_render_differently() {
    // The substitution from the original finding: same amount, same fee,
    // different recipient. It used to render identically to the honest PSBT.
    let mut hostile = Psbt::from_str(PSBT).unwrap();
    let attacker_script = Builder::new().push_int(0).push_slice(&[0xab; 32]).into_script();
    assert_ne!(hostile.unsigned_tx.output[0].script_pubkey, attacker_script);
    hostile.unsigned_tx.output[0].script_pubkey = attacker_script.clone();
    let hostile_b64 = to_base64(&hostile);

    let a = rendered_approval_screen(&parse(PSBT));
    let b = rendered_approval_screen(&parse(&hostile_b64));

    println!("{:<16} {:<40} {:<40}", "field", "honest PSBT", "substituted recipient");
    for ((k, va), (_, vb)) in a.iter().zip(b.iter()) {
        let flag = if va == vb { "same" } else { "DIFFERS" };
        println!("{k:<16} {va:<40.40} {vb:<40.40} {flag}");
    }

    // The recipient substitution is now visible as a human-readable address
    // change, not just an opaque txid difference.
    assert_ne!(
        field(&a, "Output 1"),
        field(&b, "Output 1"),
        "a substituted recipient must change what the user reads"
    );

    let attacker_address = Address::from_script(&attacker_script, NETWORK)
        .unwrap()
        .to_string();
    assert!(
        field(&b, "Output 1").contains(&attacker_address),
        "the attacker's destination address is shown to the user"
    );
    println!("-> substituted destination {attacker_address} is rendered on screen");
}

#[test]
fn t3_undecodable_psbt_refuses_signing_rather_than_rendering_a_blank_screen() {
    // The old call site did `PSBTDetails::parse(..).unwrap_or_default()`,
    // rendering an empty txid and Fee: 0 with Sign still enabled. The screen
    // decision now lives in ApprovalScreen, so it can be pinned here: this is
    // the property that actually regressed, at the layer where it regressed.
    let refused = ApprovalScreen::for_psbt("this is not a psbt", NETWORK);
    assert!(
        !refused.allows_signing(),
        "an undecodable PSBT must not offer a Sign button"
    );
    match refused {
        ApprovalScreen::Refuse { reason } => {
            assert!(reason.contains("will not be signed"));
            println!("-> {reason}");
        }
        ApprovalScreen::Approve(_) => panic!("garbage must not produce an approval screen"),
    }

    assert!(
        ApprovalScreen::for_psbt(PSBT, NETWORK).allows_signing(),
        "a well-formed PSBT is still signable"
    );
}

#[test]
fn t4_missing_witness_utxo_renders_fee_as_unknown_not_zero() {
    // psbt_details.rs used `fee_amount().unwrap_or_default()`, showing
    // "Fee: 0 SATS" whenever the fee was uncomputable.
    assert_eq!(parse(PSBT).fee, Some(1_056));

    let mut psbt = Psbt::from_str(PSBT).unwrap();
    for input in &mut psbt.inputs {
        input.witness_utxo = None;
    }
    let stripped = to_base64(&psbt);

    let details = parse(&stripped);
    let screen = rendered_approval_screen(&details);
    assert_eq!(details.fee, None, "uncomputable fee is None, not 0");
    assert_eq!(field(&screen, "Fee"), "unknown");
    assert!(!details.tx_id.is_empty());
    assert_eq!(details.outputs.len(), 1, "outputs still shown");
    println!("-> witness_utxo removed: fee is 'unknown', outputs still rendered");
}

#[test]
fn t5_every_output_of_a_multi_output_psbt_is_rendered() {
    // The attack this screen exists to stop is multi-output: a plausible
    // payment plus a drain the user never sees. Every output must appear,
    // and Total must be their sum.
    let mut psbt = Psbt::from_str(PSBT).unwrap();
    let drain_script = Builder::new().push_int(0).push_slice(&[0xcd; 32]).into_script();
    psbt.unsigned_tx.output.push(TxOut {
        value: 21_000,
        script_pubkey: drain_script.clone(),
    });
    psbt.outputs.push(bitcoin::psbt::Output::default());
    let two_output = to_base64(&psbt);

    let details = parse(&two_output);
    let screen = rendered_approval_screen(&details);
    assert_eq!(details.outputs.len(), 2, "both outputs are carried");

    let drain_address = Address::from_script(&drain_script, NETWORK).unwrap().to_string();
    let second = field(&screen, "Output 2");
    assert!(second.contains(&drain_address), "the second destination is shown");
    assert!(second.contains("21000"), "the second amount is shown");

    assert_eq!(details.total_output(), Some(50_285 + 21_000));
    assert_eq!(field(&screen, "Total"), "71285 SATS");
    for (label, value) in &screen {
        println!("{label:<10} {value}");
    }
    println!("-> both outputs rendered; Total is their sum");
}

#[test]
fn t6_non_standard_output_script_is_shown_as_a_script_not_hidden() {
    // Address::from_script fails for scripts with no address form (OP_RETURN,
    // bare multisig). The destination must still say something truthful
    // rather than render blank.
    let mut psbt = Psbt::from_str(PSBT).unwrap();
    let op_return = Builder::new()
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_RETURN)
        .push_slice(&[0xde, 0xad, 0xbe, 0xef])
        .into_script();
    assert!(Address::from_script(&op_return, NETWORK).is_err());
    psbt.unsigned_tx.output[0].script_pubkey = op_return.clone();
    let weird = to_base64(&psbt);

    let details = parse(&weird);
    let destination = &details.outputs[0].destination;
    assert!(!destination.is_empty(), "destination must never render blank");
    assert!(destination.contains("non-standard script"));
    assert!(
        destination.contains(&format!("{:x}", op_return)),
        "the raw script is shown so the user is not told a false destination"
    );
    println!("-> non-standard output rendered as: {destination}");
}

#[test]
fn t7_crafted_amounts_cannot_panic_the_parse_and_bypass_the_refusal() {
    // bdk's PsbtUtils::fee_amount panics on both of these (it does
    // `.expect("input amount must be greater than output amount")` and
    // indexes `non_witness_utxo.output[vout]` unchecked). A panic in wasm
    // traps mid-render, which would take out the refusal screen itself, so
    // the fee is computed locally with checked arithmetic instead.

    // (a) outputs exceeding inputs.
    let mut overspend = Psbt::from_str(PSBT).unwrap();
    overspend.unsigned_tx.output[0].value = u64::MAX;
    let details = parse(&to_base64(&overspend));
    assert_eq!(details.fee, None, "an impossible fee is unknown, not a panic");
    assert_eq!(field(&rendered_approval_screen(&details), "Fee"), "unknown");

    // (b) a non_witness_utxo too short for the input's vout.
    let mut short_utxo = Psbt::from_str(PSBT).unwrap();
    for input in &mut short_utxo.inputs {
        input.witness_utxo = None;
        input.non_witness_utxo = Some(Transaction {
            version: 2,
            lock_time: bitcoin::PackedLockTime::ZERO,
            input: vec![],
            output: vec![],
        });
    }
    let details = parse(&to_base64(&short_utxo));
    assert_eq!(details.fee, None, "out-of-range vout is unknown, not a panic");

    // (c) output amounts that overflow u64 when summed: Total says unknown
    // while each real per-output amount is still shown.
    let mut overflow = Psbt::from_str(PSBT).unwrap();
    overflow.unsigned_tx.output[0].value = u64::MAX;
    overflow.unsigned_tx.output.push(TxOut {
        value: 10,
        script_pubkey: overflow.unsigned_tx.output[0].script_pubkey.clone(),
    });
    overflow.outputs.push(bitcoin::psbt::Output::default());
    let details = parse(&to_base64(&overflow));
    let screen = rendered_approval_screen(&details);
    assert_eq!(details.total_output(), None);
    assert_eq!(field(&screen, "Total"), "unknown", "an overflowing total never understates");
    assert!(field(&screen, "Output 1").contains(&u64::MAX.to_string()));
    assert!(field(&screen, "Output 2").contains("10 SATS"));

    // All three still reached a real approval screen rather than trapping.
    println!("-> crafted amounts render as 'unknown' instead of panicking mid-render");
}
