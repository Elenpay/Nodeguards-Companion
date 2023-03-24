use bitcoin::Network;

pub mod psbt_details;
pub mod signer;
pub mod storage;
pub mod utils;
pub mod wallet;

#[cfg(feature = "regtest")]
pub const NETWORK: Network = Network::Regtest;

#[cfg(feature = "testnet")]
pub const NETWORK: Network = Network::Testnet;

#[cfg(feature = "signet")]
pub const NETWORK: Network = Network::Signet;

#[cfg(not(any(feature = "regtest", feature = "testnet", feature = "signet")))]
pub const NETWORK: Network = Network::Bitcoin;
