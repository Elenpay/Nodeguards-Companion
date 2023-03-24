pub struct PSBTWithWallet {
    pub psbt: String,
    pub wallet_name: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PasswordFor {
    ImportingMnemonic,
    SigningPSBT,
}
