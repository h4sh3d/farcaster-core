//! Protocol messages exchanged between swap daemons

use strict_encoding::{StrictDecode, StrictEncode};

use crate::blockchain::Onchain;
use crate::crypto::{Commitment, Keys, SharedPrivateKeys, Signatures};
use crate::role::{Acc, Arbitrating};
use crate::swap::Swap;

/// Trait for defining inter-daemon communication messages.
pub trait ProtocolMessage: StrictEncode + StrictDecode {}

/// `commit_alice_session_params` forces Alice to commit to the result of her cryptographic setup
/// before receiving Bob's setup. This is done to remove adaptive behavior.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct CommitAliceSessionParams<Ctx: Swap> {
    /// Commitment to `Ab` curve point
    pub buy: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Ac` curve point
    pub cancel: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Ar` curve point
    pub refund: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Ap` curve point
    pub punish: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Ta` curve point
    pub adaptor: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `k_v^a` scalar
    pub spend: <Ctx::Ac as Commitment>::Commitment,
    /// Commitment to `K_s^a` curve point
    pub view: <Ctx::Ac as Commitment>::Commitment,
}

impl<Ctx> std::fmt::Display for CommitAliceSessionParams<Ctx>
where
    Ctx: Swap,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        write!(f, "{}", self)
    }
}

impl<Ctx> ProtocolMessage for CommitAliceSessionParams<Ctx> where Ctx: Swap {}

/// `commit_bob_session_params` forces Bob to commit to the result of his cryptographic setup
/// before receiving Alice's setup. This is done to remove adaptive behavior.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct CommitBobSessionParams<Ctx: Swap> {
    /// Commitment to `Bb` curve point
    pub buy: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Bc` curve point
    pub cancel: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Br` curve point
    pub refund: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `Tb` curve point
    pub adaptor: <Ctx::Ar as Commitment>::Commitment,
    /// Commitment to `k_v^b` scalar
    pub spend: <Ctx::Ac as Commitment>::Commitment,
    /// Commitment to `K_s^b` curve point
    pub view: <Ctx::Ac as Commitment>::Commitment,
}

impl<Ctx> ProtocolMessage for CommitBobSessionParams<Ctx> where Ctx: Swap {}

/// `reveal_alice_session_params` reveals the parameters commited by the
/// `commit_alice_session_params` message.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct RevealAliceSessionParams<Ctx: Swap> {
    /// The buy `Ab` public key
    pub buy: <Ctx::Ar as Keys>::PublicKey,
    /// The cancel `Ac` public key
    pub cancel: <Ctx::Ar as Keys>::PublicKey,
    /// The refund `Ar` public key
    pub refund: <Ctx::Ar as Keys>::PublicKey,
    /// The punish `Ap` public key
    pub punish: <Ctx::Ar as Keys>::PublicKey,
    /// The `Ta` adaptor public key
    pub adaptor: <Ctx::Ar as Keys>::PublicKey,
    /// The destination Bitcoin address
    pub address: <Ctx::Ar as Arbitrating>::Address,
    /// The `K_v^a` view private key
    pub spend: <Ctx::Ac as Keys>::PublicKey,
    /// The `K_s^a` spend public key
    pub view: <Ctx::Ac as SharedPrivateKeys<Acc>>::SharedPrivateKey,
    /// The cross-group discrete logarithm zero-knowledge proof
    pub proof: Ctx::Proof,
}

impl<Ctx> ProtocolMessage for RevealAliceSessionParams<Ctx> where Ctx: Swap {}

/// `reveal_bob_session_params` reveals the parameters commited by the `commit_bob_session_params`
/// message.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct RevealBobSessionParams<Ctx: Swap> {
    /// The buy `Bb` public key
    pub buy: <Ctx::Ar as Keys>::PublicKey,
    /// The cancel `Bc` public key
    pub cancel: <Ctx::Ar as Keys>::PublicKey,
    /// The refund `Br` public key
    pub refund: <Ctx::Ar as Keys>::PublicKey,
    /// The `Tb` adaptor public key
    pub adaptor: <Ctx::Ar as Keys>::PublicKey,
    /// The refund Bitcoin address
    pub address: <Ctx::Ar as Arbitrating>::Address,
    /// The `K_v^b` view private key
    pub spend: <Ctx::Ac as Keys>::PublicKey,
    /// The `K_s^b` spend public key
    pub view: <Ctx::Ac as SharedPrivateKeys<Acc>>::SharedPrivateKey,
    /// The cross-group discrete logarithm zero-knowledge proof
    pub proof: Ctx::Proof,
}

impl<Ctx> ProtocolMessage for RevealBobSessionParams<Ctx> where Ctx: Swap {}

/// `core_arbitrating_setup` sends the `lock (b)`, `cancel (d)` and `refund (e)` arbritrating
/// transactions from Bob to Alice, as well as Bob's signature for the `cancel (d)` transaction.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct CoreArbitratingSetup<Ctx: Swap> {
    /// The arbitrating `lock (b)` transaction
    pub lock: <Ctx::Ar as Onchain>::PartialTransaction,
    /// The arbitrating `cancel (d)` transaction
    pub cancel: <Ctx::Ar as Onchain>::PartialTransaction,
    /// The arbitrating `refund (e)` transaction
    pub refund: <Ctx::Ar as Onchain>::PartialTransaction,
    /// The `Bc` `cancel (d)` signature
    pub cancel_sig: <Ctx::Ar as Signatures>::Signature,
}

impl<Ctx> ProtocolMessage for CoreArbitratingSetup<Ctx> where Ctx: Swap {}

/// `refund_procedure_signatures` is intended to transmit Alice's signature for the `cancel (d)`
/// transaction and Alice's adaptor signature for the `refund (e)` transaction. Uppon reception Bob
/// must validate the signatures.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct RefundProcedureSignatures<Ctx: Swap> {
    /// The `Ac` `cancel (d)` signature
    pub cancel_sig: <Ctx::Ar as Signatures>::Signature,
    /// The `Ar(Tb)` `refund (e)` adaptor signature
    pub refund_adaptor_sig: <Ctx::Ar as Signatures>::AdaptorSignature,
}

impl<Ctx> ProtocolMessage for RefundProcedureSignatures<Ctx> where Ctx: Swap {}

/// `buy_procedure_signature`is intended to transmit Bob's adaptor signature for the `buy (c)`
/// transaction and the transaction itself. Uppon reception Alice must validate the transaction and
/// the adaptor signature.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct BuyProcedureSignature<Ctx: Swap> {
    /// The arbitrating `buy (c)` transaction
    pub buy: <Ctx::Ar as Onchain>::PartialTransaction,
    /// The `Bb(Ta)` `buy (c)` adaptor signature
    pub buy_adaptor_sig: <Ctx::Ar as Signatures>::AdaptorSignature,
}

impl<Ctx> ProtocolMessage for BuyProcedureSignature<Ctx> where Ctx: Swap {}

/// `abort` is an `OPTIONAL` courtesy message from either swap partner to inform the counterparty
/// that they have aborted the swap with an `OPTIONAL` message body to provide the reason.
#[derive(Clone, Debug, StrictDecode, StrictEncode)]
#[strict_encoding_crate(strict_encoding)]
pub struct Abort {
    /// OPTIONAL `body`: error code | string
    pub error_body: Option<String>,
}

impl ProtocolMessage for Abort {}
