#[cfg(not(feature = "liquid"))]
pub use groestlcoin::util::address;
#[cfg(not(feature = "liquid"))] // use regular Groestlcoin data structures
pub use groestlcoin::{Block, BlockHeader, OutPoint, Transaction, TxIn, TxOut};

#[cfg(feature = "liquid")]
pub use elements::address;
#[cfg(feature = "liquid")]
pub use elements::{confidential, Address, Block, BlockHeader, OutPoint, Transaction, TxIn, TxOut};

use groestlcoin::blockdata::constants::genesis_block;
use groestlcoin::network::constants::Network as BNetwork;
use groestlcoin::util::hash::BitcoinHash;

#[cfg(not(feature = "liquid"))]
pub type Value = u64;
#[cfg(feature = "liquid")]
pub use confidential::Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    Groestlcoin,
    Testnet,
    Regtest,

    #[cfg(feature = "liquid")]
    Liquid,
    #[cfg(feature = "liquid")]
    LiquidRegtest,
}

impl Network {
    pub fn genesis_hash(&self) -> groestlcoin::BlockHash {
        let block = genesis_block(BNetwork::from(self));
        block.bitcoin_hash()
    }

    pub fn magic(&self) -> u32 {
        match self {
            Network::Groestlcoin => 0xD4B4BEF9,
            Network::Testnet => 0x0709110B,
            Network::Regtest => 0xDAB5BFFA,

            #[cfg(feature = "liquid")]
            Network::Liquid => 0xDAB5BFFA,
            #[cfg(feature = "liquid")]
            Network::LiquidRegtest => 0xDAB5BFFA,
        }
    }

    #[cfg(feature = "liquid")]
    pub fn address_params(&self) -> &'static address::AddressParams {
        // Liquid regtest uses elements's address params
        match self {
            Network::Liquid => &address::AddressParams::LIQUID,
            Network::LiquidRegtest => &address::AddressParams::ELEMENTS,
            _ => panic!("the liquid-only address_params() called with non-liquid network"),
        }
    }

    pub fn names() -> Vec<String> {
        #[cfg(not(feature = "liquid"))]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
        ];

        #[cfg(feature = "liquid")]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
            "liquid".to_string(),
            "liquidregtest".to_string(),
        ];
    }
}

impl From<&str> for Network {
    fn from(network_name: &str) -> Self {
        match network_name {
            "mainnet" => Network::Groestlcoin,
            "testnet" => Network::Testnet,
            "regtest" => Network::Regtest,

            #[cfg(feature = "liquid")]
            "liquid" => Network::Liquid,
            #[cfg(feature = "liquid")]
            "liquidregtest" => Network::LiquidRegtest,

            _ => panic!("unsupported Groestlcoin network: {:?}", network_name),
        }
    }
}

impl From<&Network> for BNetwork {
    fn from(network: &Network) -> Self {
        match network {
            Network::Groestlcoin => BNetwork::Groestlcoin,
            Network::Testnet => BNetwork::Testnet,
            Network::Regtest => BNetwork::Regtest,

            #[cfg(feature = "liquid")]
            Network::Liquid => BNetwork::Groestlcoin, // @FIXME
            #[cfg(feature = "liquid")]
            Network::LiquidRegtest => BNetwork::Regtest, // @FIXME
        }
    }
}

impl From<&BNetwork> for Network {
    fn from(network: &BNetwork) -> Self {
        match network {
            #[cfg(not(feature = "liquid"))]
            BNetwork::Groestlcoin => Network::Groestlcoin,
            #[cfg(not(feature = "liquid"))]
            BNetwork::Regtest => Network::Regtest,

            #[cfg(feature = "liquid")]
            BNetwork::Groestlcoin => Network::Liquid, // @FIXME
            #[cfg(feature = "liquid")]
            BNetwork::Regtest => Network::LiquidRegtest, // @FIXME
            BNetwork::Testnet => Network::Testnet, // @FIXME
        }
    }
}
