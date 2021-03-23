//! Defines the interface a blockchain must implement
//!
//! A blockchain must identify the block chain (or equivalent), e.g. with the genesis hash, and the
//! asset, e.g. for Etherum blockchain assets can be eth or dai.

/// Base trait for defining a blockchain and its asset type.
pub trait Blockchain: Copy {
    /// Type for the traded asset unit
    type AssetUnit: Copy;

    /// Create a new blockchain
    fn new() -> Self;
}

/// Defines the types a blockchain needs to interact onchain, i.e. the transaction types.
pub trait Onchain {
    /// Defines the transaction format used to transfer partial transaction between participant for
    /// the arbitrating blockchain
    type PartialTransaction;

    /// Defines the finalized transaction format for the arbitrating blockchain
    type Transaction;
}

/// Define the unit type used for setting/validating blockchain fees.
pub trait FeeUnit {
    /// Type for describing the fees of a blockchain
    type FeeUnit: Clone + PartialOrd + PartialEq;
}

/// Define the type of errors a fee strategy can encounter during calculation, application, and
/// validation of fees on a partial transaction.
#[derive(Debug, PartialEq)]
pub enum FeeStrategyError {
    MissingInputsMetadata,
    AmountOfFeeTooHigh,
    NotEnoughAssets,
    MultiOutputUnsupported,
}

/// Defines how to set the fees when a strategy allows multiple possibilities.
#[derive(Debug, Clone, Copy)]
pub enum FeePolitic {
    /// Set the fees at the minimum allowed by the strategy
    Aggressive,
    /// Set the fees at the maximum allowed by the strategy
    Conservative,
}

/// A fee strategy to be applied on an arbitrating transaction.
///
/// As described in the specifications a fee strategy can be: fixe, range, or more advanced form
/// of fee calculation.
///
/// A fee strategy is included in an offer, so Alice and Bob can verify that transactions are valid
/// upon reception by the other participant.
pub trait FeeStrategy: FeeUnit {
    /// Create a new fixed fee strategy
    fn fixed_fee(fee: Self::FeeUnit) -> Self;

    /// Create a new fee strategy that applies fees according to a range
    fn range_fee(fee_low: Self::FeeUnit, fee_high: Self::FeeUnit) -> Self;
}

/// Enable fee management for an arbitrating blockchain. This trait require implementing the
/// Onchain role to have access to transaction associated type and to specify the concrete fee
/// strategy type to use.
pub trait Fee: Onchain + Blockchain {
    /// The fee strategy concrete type
    type FeeStrategy: FeeStrategy + Clone;

    /// Calculates and sets the fees on the given transaction and return the amount of fees set in
    /// the blockchain native amount format.
    fn set_fees(
        tx: &mut Self::PartialTransaction,
        strategy: &Self::FeeStrategy,
        politic: FeePolitic,
    ) -> Result<Self::AssetUnit, FeeStrategyError>;

    /// Validates that the fees for the given transaction are set accordingly to the strategy
    fn validate_fee(
        tx: &Self::PartialTransaction,
        strategy: &Self::FeeStrategy,
        politic: FeePolitic,
    ) -> Result<bool, FeeStrategyError>;
}

/// Defines a blockchain network, identifies in which context the system interacts with the
/// blockchain.
#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum Network {
    /// Represents a real asset on his valuable network
    Mainnet,
    /// Represents non-valuable assets on test networks
    Testnet,
    /// Local and private testnets
    Local,
}