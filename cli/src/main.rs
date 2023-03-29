use std::env;

use signer::{signer::*, wallet::Wallet};

fn main() {
    let mut args = env::args().skip(1);

    // let mut wallet = Wallet::from_seed_str("social mango annual basic work brain economy one safe physical junk other toy valid load cook napkin maple runway island oil fan legend stem");
    let mut wallet = Wallet::default();
    wallet.from_seed_str("wallet 1", "solar goat auto bachelor chronic input twin depth fork scale divorce fury mushroom column image sauce car public artist announce treat spend jacket physical", "Qwerty123").unwrap();

    let signed_psbt = decode_psbt_and_sign(&args.next().unwrap(), &mut wallet, "Querty123");
    println!("{}", signed_psbt.unwrap())
}
