use bdk::psbt::PsbtUtils;
use bitcoin::hashes::hex::ToHex;
use bitcoin::psbt::PartiallySignedTransaction;
use std::str::FromStr;

pub enum Action {
    ChannelRequest,
    Withdrawal,
}

pub struct PSBTDetails {
    pub tx_id: String,
    pub fee: u64,
}

impl PSBTDetails {
    pub fn from_str(psbt_64: &str) -> Self {
        let psbt = PartiallySignedTransaction::from_str(psbt_64).unwrap();
        let tx_id = psbt.clone().extract_tx().txid().to_hex();
        let fee = psbt.fee_amount().unwrap_or_default();
        Self { tx_id, fee }
    }
}
