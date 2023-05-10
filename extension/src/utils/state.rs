pub struct PSBTWithWallet {
    pub psbt: String,
    pub wallet_name: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PasswordFor {
    ImportingSecret,
    UnlockingApp,
}
