use std::env;

use signer::signer::*;

fn main() {
    let mut args = env::args().skip(1);
    decode_psbt_and_sign(&args.next().unwrap())
}
