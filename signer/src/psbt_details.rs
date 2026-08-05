use bitcoin::hashes::hex::ToHex;
use bitcoin::psbt::PartiallySignedTransaction;
use bitcoin::{Address, Network, TxOut};
use std::str::FromStr;

pub enum Action {
    ChannelRequest,
    Withdrawal,
}

pub struct PSBTOutput {
    pub destination: String,
    pub amount: u64,
}

pub struct PSBTDetails {
    pub tx_id: String,
    /// `None` when the fee cannot be computed from the PSBT — missing UTXO
    /// information, or amounts that do not add up. Must be rendered as
    /// unknown, never as 0.
    pub fee: Option<u64>,
    pub outputs: Vec<PSBTOutput>,
    pub network: Network,
}

/// What the approval screen is allowed to do with a given PSBT. A PSBT that
/// cannot be decoded has nothing trustworthy to display, so signing it is
/// refused rather than offered over a blank screen (NGC-01).
pub enum ApprovalScreen {
    Approve(PSBTDetails),
    Refuse { reason: String },
}

impl ApprovalScreen {
    pub fn for_psbt(psbt_64: &str, network: Network) -> Self {
        match PSBTDetails::parse(psbt_64, network) {
            Ok(details) => Self::Approve(details),
            Err(e) => Self::Refuse {
                reason: format!("This PSBT could not be decoded and will not be signed: {e}"),
            },
        }
    }

    pub const fn allows_signing(&self) -> bool {
        matches!(self, Self::Approve(_))
    }
}

/// Total of an input's previous output, or `None` if the PSBT does not carry
/// enough information to determine it. Deliberately avoids `bdk`'s
/// `PsbtUtils::fee_amount`, which panics on crafted PSBTs (out-of-range
/// `vout`, outputs exceeding inputs) — a panic in wasm traps mid-render and
/// would bypass the refusal screen entirely.
fn input_value(psbt: &PartiallySignedTransaction, index: usize) -> Option<u64> {
    let input = psbt.inputs.get(index)?;
    if let Some(witness_utxo) = &input.witness_utxo {
        return Some(witness_utxo.value);
    }
    let previous_tx = input.non_witness_utxo.as_ref()?;
    let vout = psbt.unsigned_tx.input.get(index)?.previous_output.vout;
    previous_tx
        .output
        .get(vout as usize)
        .map(|previous: &TxOut| previous.value)
}

fn checked_total(values: impl IntoIterator<Item = u64>) -> Option<u64> {
    values
        .into_iter()
        .try_fold(0_u64, |total, value| total.checked_add(value))
}

impl PSBTDetails {
    /// Derives everything the approval screen displays from the PSBT itself,
    /// so the requesting page has no influence over what the user reads
    /// before signing (NGC-01).
    pub fn parse(psbt_64: &str, network: Network) -> Result<Self, anyhow::Error> {
        let psbt = PartiallySignedTransaction::from_str(psbt_64)?;
        let tx_id = psbt.clone().extract_tx().txid().to_hex();
        let outputs: Vec<PSBTOutput> = psbt
            .unsigned_tx
            .output
            .iter()
            .map(|output| PSBTOutput {
                destination: Address::from_script(&output.script_pubkey, network)
                    .map_or_else(
                        |_| format!("non-standard script {}", output.script_pubkey.to_hex()),
                        |address| address.to_string(),
                    ),
                amount: output.value,
            })
            .collect();

        let total_in = (0..psbt.inputs.len())
            .map(|index| input_value(&psbt, index))
            .collect::<Option<Vec<u64>>>()
            .and_then(checked_total);
        let total_out = checked_total(psbt.unsigned_tx.output.iter().map(|output| output.value));
        let fee = total_in
            .zip(total_out)
            .and_then(|(total_in, total_out)| total_in.checked_sub(total_out));

        Ok(Self {
            tx_id,
            fee,
            outputs,
            network,
        })
    }

    /// `None` if the amounts overflow, which only a crafted PSBT can do. The
    /// per-output rows still carry the real values.
    pub fn total_output(&self) -> Option<u64> {
        checked_total(self.outputs.iter().map(|output| output.amount))
    }

    /// The exact rows the approval screen renders, in order. Lives here so
    /// that what the tests assert on is what the UI displays.
    pub fn display_fields(&self) -> Vec<(String, String)> {
        let mut fields = vec![("Tx Id".to_string(), self.tx_id.clone())];
        for (index, output) in self.outputs.iter().enumerate() {
            fields.push((
                format!("Output {}", index + 1),
                format!("{} SATS to {}", output.amount, output.destination),
            ));
        }
        fields.push(("Total".to_string(), format_sats(self.total_output())));
        fields.push(("Fee".to_string(), format_sats(self.fee)));
        fields.push(("Network".to_string(), self.network.to_string()));
        fields
    }
}

fn format_sats(amount: Option<u64>) -> String {
    amount.map_or_else(|| "unknown".to_string(), |amount| format!("{amount} SATS"))
}
