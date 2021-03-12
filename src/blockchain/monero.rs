//! Defines and implements all the traits for Monero

use monero::cryptonote::hash::Hash;
use monero::network::Network;
use monero::util::key::PrivateKey;
use monero::util::key::PublicKey;

use lightning_encoding::{strategies, Strategy, LightningEncode};

use crate::blockchain::Blockchain;
use crate::role::Accordant;

#[derive(Clone, Copy)]
pub struct Monero;

impl Blockchain for Monero {
    /// Type for the traded asset unit
    type AssetUnit = u64;

    /// Type of the blockchain identifier
    type Id = String;

    /// Type of the chain identifier
    type ChainId = Network;

    /// Returns the blockchain identifier
    fn id(&self) -> String {
        String::from("xmr")
    }

    /// Returns the chain identifier
    fn chain_id(&self) -> Network {
        Network::Mainnet
    }

    /// Create a new Bitcoin blockchain
    fn new() -> Self {
        Monero {}
    }
}

#[derive(Debug, LightningEncode)]
pub struct CommitmentHash(Hash);

impl Strategy for CommitmentHash {
    type Strategy = strategies::AsStrict;
}

impl Accordant for Monero {
    type PrivateKey = PrivateKey;
    type PublicKey = PublicKey;
    type Commitment = CommitmentHash;
}
