use bdk::psbt::PsbtUtils;
use bitcoin::hashes::hex::ToHex;
use bitcoin::psbt::PartiallySignedTransaction;
use std::str::FromStr;

pub enum Action {
    ChannelRequest,
    Withdrawal,
}

#[derive(Default)]
pub struct PSBTDetails {
    pub tx_id: String,
    pub fee: u64,
}

impl FromStr for PSBTDetails {
    type Err = anyhow::Error;

    fn from_str(psbt_64: &str) -> Result<Self, Self::Err> {
        let psbt = PartiallySignedTransaction::from_str(psbt_64)?;
        let tx_id = psbt.clone().extract_tx().txid().to_hex();
        let fee = psbt.fee_amount().unwrap_or_default();
        Ok(Self { tx_id, fee })
    }
}
