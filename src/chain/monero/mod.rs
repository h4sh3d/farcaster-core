//! Defines and implements all the traits for Monero

use crate::blockchain::{Address, Asset};
use crate::consensus::AsCanonicalBytes;
use crate::crypto::{
    self, AccordantKeyId, GenerateKey, GenerateSharedKey, Keys, SharedKeyId, SharedPrivateKeys,
};
use crate::role::Accordant;

use monero::cryptonote::hash::Hash;
use monero::util::key::{PrivateKey, PublicKey};

use std::fmt::{self, Debug, Display, Formatter};

pub mod tasks;

pub const SHARED_VIEW_KEY: u16 = 0x01;
pub const SHARED_KEY_BITS: usize = 252;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Monero;

impl std::str::FromStr for Monero {
    type Err = crate::consensus::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monero" => Ok(Monero),
            _ => Err(crate::consensus::Error::UnknownType),
        }
    }
}

impl Display for Monero {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        println!("xmr");
        Ok(())
    }
}

impl Asset for Monero {
    /// Type for the traded asset unit
    type AssetUnit = u64;

    /// Create a new Bitcoin blockchain
    fn new() -> Self {
        Monero {}
    }

    fn from_u32(bytes: u32) -> Option<Self> {
        match bytes {
            0x80000080 => Some(Self::new()),
            _ => None,
        }
    }

    fn to_u32(&self) -> u32 {
        0x80000080
    }
}

impl Accordant for Monero {}

impl Address for Monero {
    type Address = monero::Address;
}

impl Keys for Monero {
    /// Private key type for the blockchain
    type PrivateKey = PrivateKey;

    /// Public key type for the blockchain
    type PublicKey = PublicKey;

    fn extra_keys() -> Vec<u16> {
        // No extra key
        vec![]
    }
}

impl AsCanonicalBytes for PrivateKey {
    fn as_canonical_bytes(&self) -> Vec<u8> {
        self.to_bytes().into()
    }
}

impl AsCanonicalBytes for PublicKey {
    fn as_canonical_bytes(&self) -> Vec<u8> {
        self.as_bytes().into()
    }
}

impl SharedPrivateKeys for Monero {
    type SharedPrivateKey = PrivateKey;

    fn shared_keys() -> Vec<SharedKeyId> {
        // Share one key: the private view key
        vec![SharedKeyId::new(SHARED_VIEW_KEY)]
    }
}

pub fn private_spend_from_seed<T: AsRef<[u8]>>(seed: T) -> Result<PrivateKey, crypto::Error> {
    let mut bytes = Vec::from(b"farcaster_priv_spend".as_ref());
    bytes.extend_from_slice(&seed.as_ref());

    let mut key = Hash::hash(&bytes).to_fixed_bytes();
    key[31] &= 0b0000_1111; // Chop off bits that might be greater than the curve modulus

    PrivateKey::from_slice(&key).map_err(|e| crypto::Error::new(e))
}

#[derive(Clone, Debug)]
pub struct Wallet {
    seed: [u8; 32],
}

impl Wallet {
    pub fn new(seed: [u8; 32]) -> Self {
        Self { seed }
    }
}

impl GenerateKey<PublicKey, AccordantKeyId> for Wallet {
    fn get_pubkey(&self, key_id: AccordantKeyId) -> Result<PublicKey, crypto::Error> {
        match key_id {
            AccordantKeyId::Spend => Ok(PublicKey::from_private_key(&private_spend_from_seed(
                &self.seed,
            )?)),
            AccordantKeyId::Extra(_) => Err(crypto::Error::UnsupportedKey),
        }
    }
}

impl GenerateSharedKey<PrivateKey> for Wallet {
    fn get_shared_key(&self, key_id: SharedKeyId) -> Result<PrivateKey, crypto::Error> {
        match key_id.id() {
            SHARED_VIEW_KEY => {
                let mut bytes = Vec::from(b"farcaster_priv_view".as_ref());
                bytes.extend_from_slice(&self.seed.as_ref());
                Ok(Hash::hash(&bytes).as_scalar())
            }
            _ => Err(crypto::Error::UnsupportedKey),
        }
    }
}
