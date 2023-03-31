use std::env;

use signer::{signer::*, wallet::Wallet};

fn main() {
    let mut args = env::args().skip(1);

    let mut wallet = Wallet::default();
    //wallet.from_seed_str("wallet 1", "solar goat auto bachelor chronic input twin depth fork scale divorce fury mushroom column image sauce car public artist announce treat spend jacket physical", "Qwerty123").unwrap();
    wallet.from_xprv_str("wallet 1", "tprv8ZgxMBicQKsPduvXYAnkop1b1UoAY2pS68pe9jHuJwuMvx6G5sh4C67peYZkRawdBWbMbfoybgQJ3g8nTZAezEeHyaW9A9UjtpTRmSyJwUn", "m/48'/1'/1'", "Qwerty123").unwrap();
    let signed_psbt = decode_psbt_and_sign(&args.next().unwrap(), &mut wallet, "Qwerty123");
    println!("{}", signed_psbt.unwrap())
}
