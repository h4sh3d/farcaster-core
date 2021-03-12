//! Protocol messages exchanged between swap daemons

use lightning_encoding::{LightningDecode, LightningEncode, lightning_serialize};

use crate::crypto::{Crypto, CryptoEngine};
use crate::role::{Accordant, Arbitrating};

//use crate::blockchain::{bitcoin::Bitcoin, monero::Monero};
//use crate::crypto::{ECDSAScripts};

//pub enum Message<Ar, Ac, C>
//where
//    Ar: Arbitrating + Crypto<C>,
//    Ac: Accordant,
//    C: CryptoEngine,
//{
//    CommitAliceSessionParams(CommitAliceSessionParams<Ar, Ac, C>),
//}

/// Trait for defining inter-daemon communication messages.
pub trait ProtocolMessage: LightningEncode {} // LightningDecode + 

/// `commit_alice_session_params` forces Alice to commit to the result of her cryptographic setup
/// before receiving Bob's setup. This is done to remove adaptive behavior.
#[derive(Debug, LightningEncode)]
pub struct CommitAliceSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
    /// Commitment to `Ab` curve point
    pub buy: Ar::Commitment,
    /// Commitment to `Ac` curve point
    pub cancel: Ar::Commitment,
    /// Commitment to `Ar` curve point
    pub refund: Ar::Commitment,
    /// Commitment to `Ap` curve point
    pub punish: Ar::Commitment,
    /// Commitment to `Ta` curve point
    pub adaptor: Ar::Commitment,
    /// Commitment to `k_v^a` scalar
    pub spend: Ac::Commitment,
    /// Commitment to `K_s^a` curve point
    pub view: Ac::Commitment,
}

impl<Ar, Ac, C> ProtocolMessage for CommitAliceSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
}

/// `commit_bob_session_params` forces Bob to commit to the result of his cryptographic setup
/// before receiving Alice's setup. This is done to remove adaptive behavior.
#[derive(Debug, LightningEncode)]
pub struct CommitBobSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
    /// Commitment to `Bb` curve point
    pub buy: Ar::Commitment,
    /// Commitment to `Bc` curve point
    pub cancel: Ar::Commitment,
    /// Commitment to `Br` curve point
    pub refund: Ar::Commitment,
    /// Commitment to `Tb` curve point
    pub adaptor: Ar::Commitment,
    /// Commitment to `k_v^b` scalar
    pub spend: Ac::Commitment,
    /// Commitment to `K_s^b` curve point
    pub view: Ac::Commitment,
}

impl<Ar, Ac, C> ProtocolMessage for CommitBobSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
}

/// `reveal_alice_session_params` reveals the parameters commited by the
/// `commit_alice_session_params` message.
#[derive(Debug, LightningEncode)]
pub struct RevealAliceSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
    /// The buy `Ab` public key
    pub buy: Ar::PublicKey,
    /// The cancel `Ac` public key
    pub cancel: Ar::PublicKey,
    /// The refund `Ar` public key
    pub refund: Ar::PublicKey,
    /// The punish `Ap` public key
    pub punish: Ar::PublicKey,
    /// The `Ta` adaptor public key
    pub adaptor: Ar::PublicKey,
    /// The destination Bitcoin address
    pub address: Ar::Address,
    /// The `K_v^a` view private key
    pub spend: Ac::PublicKey,
    /// The `K_s^a` spend public key
    pub view: Ac::PrivateKey,
    /// The cross-group discrete logarithm zero-knowledge proof
    pub proof: Option<String>,
}

impl<Ar, Ac, C> ProtocolMessage for RevealAliceSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
}

/// `reveal_bob_session_params` reveals the parameters commited by the `commit_bob_session_params`
/// message.
#[derive(Debug, LightningEncode)]
pub struct RevealBobSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
    /// The buy `Bb` public key
    pub buy: Ar::PublicKey,
    /// The cancel `Bc` public key
    pub cancel: Ar::PublicKey,
    /// The refund `Br` public key
    pub refund: Ar::PublicKey,
    /// The `Tb` adaptor public key
    pub adaptor: Ar::PublicKey,
    /// The refund Bitcoin address
    pub address: Ar::Address,
    /// The `K_v^b` view private key
    pub spend: Ac::PublicKey,
    /// The `K_s^b` spend public key
    pub view: Ac::PrivateKey,
    /// The cross-group discrete logarithm zero-knowledge proof
    pub proof: Option<String>,
}

impl<Ar, Ac, C> ProtocolMessage for RevealBobSessionParams<Ar, Ac, C>
where
    Ar: Arbitrating + Crypto<C>,
    Ac: Accordant,
    C: CryptoEngine,
{
}

/// `core_arbitrating_setup` sends the `lock (b)`, `cancel (d)` and `refund (e)` arbritrating
/// transactions from Bob to Alice, as well as Bob's signature for the `cancel (d)` transaction.
#[derive(Debug, LightningEncode)]
pub struct CoreArbitratingSetup<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
    /// The arbitrating `lock (b)` transaction
    pub lock: Ar::Transaction,
    /// The arbitrating `cancel (d)` transaction
    pub cancel: Ar::Transaction,
    /// The arbitrating `refund (e)` transaction
    pub refund: Ar::Transaction,
    /// The `Bc` `cancel (d)` signature
    pub cancel_sig: Ar::Signature,
}

impl<Ar, C> ProtocolMessage for CoreArbitratingSetup<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
}

/// `refund_procedure_signatures` is intended to transmit Alice's signature for the `cancel (d)`
/// transaction and Alice's adaptor signature for the `refund (e)` transaction. Uppon reception Bob
/// must validate the signatures.
#[derive(Debug, LightningEncode)]
pub struct RefundProcedureSignatures<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
    /// The `Ac` `cancel (d)` signature
    pub cancel_sig: Ar::Signature,
    /// The `Ar(Tb)` `refund (e)` adaptor signature
    pub refund_adaptor_sig: Ar::AdaptorSignature,
}

impl<Ar, C> ProtocolMessage for RefundProcedureSignatures<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
}

/// `buy_procedure_signature`is intended to transmit Bob's adaptor signature for the `buy (c)`
/// transaction and the transaction itself. Uppon reception Alice must validate the transaction and
/// the adaptor signature.
#[derive(Debug, LightningEncode)]
pub struct BuyProcedureSignature<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
    /// The arbitrating `buy (c)` transaction
    pub buy: Ar::Transaction,
    /// The `Bb(Ta)` `buy (c)` adaptor signature
    pub buy_adaptor_sig: Ar::AdaptorSignature,
}

impl<Ar, C> ProtocolMessage for BuyProcedureSignature<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
}

/// `abort` is an `OPTIONAL` courtesy message from either swap partner to inform the counterparty
/// that they have aborted the swap with an `OPTIONAL` message body to provide the reason.
#[derive(Debug, LightningEncode)]
pub struct Abort {
    /// OPTIONAL `body`: error code | string
    pub error_body: Option<String>,
}

impl ProtocolMessage for Abort {}








#[derive(Debug, LightningEncode)]
pub struct TestMsg<Ar, C>
where
    Ar: Arbitrating + Crypto<C>,
    C: CryptoEngine,
{
    /// The `Ar(Tb)` `refund (e)` adaptor signature
    pub refund_adaptor_sig: Ar::AdaptorSignature,
}






#[cfg(test)]
mod tests {

    use bitcoin::blockdata::transaction::Transaction;
    use bitcoin::util::psbt::PartiallySignedTransaction;
    use secp256k1::key::PublicKey;
    use secp256k1::Signature;

    use lightning_encoding::{lightning_serialize};

    use super::*;
    use crate::blockchain::bitcoin::{Bitcoin, ECDSAAdaptorSig, PDLEQ};
    use crate::crypto::ECDSAScripts;

    #[test]
    fn create_abort_message() {
        let _ = Abort {
            error_body: Some(String::from("An error occured ;)")),
        };
    }

    //#[test]
    //fn create_buy_procedure_signature_message() {
    //    let ecdsa_sig = "3045022100b75f569de3e57f4f445bcf9e42be9e5b5128f317ab86e451fdfe7be5ffd6a7da0220776b30307b5d761512635dc0394573be7fe17b5300b160340dae370b641bc4ca";

    //    let tx = Transaction {
    //        version: 2,
    //        lock_time: 0,
    //        input: Vec::new(),
    //        output: Vec::new(),
    //    };

    //    let sig =
    //        Signature::from_der(&hex::decode(ecdsa_sig).expect("HEX decode should work here"))
    //            .expect("Parse DER should work here");

    //    let point = PublicKey::from_slice(&[
    //        0x02, 0xc6, 0x6e, 0x7d, 0x89, 0x66, 0xb5, 0xc5, 0x55, 0xaf, 0x58, 0x05, 0x98, 0x9d,
    //        0xa9, 0xfb, 0xf8, 0xdb, 0x95, 0xe1, 0x56, 0x31, 0xce, 0x35, 0x8c, 0x3a, 0x17, 0x10,
    //        0xc9, 0x62, 0x67, 0x90, 0x63,
    //    ])
    //    .expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");

    //    let pdleq = PDLEQ;

    //    let _ = BuyProcedureSignature::<Bitcoin, ECDSAScripts> {
    //        buy: (PartiallySignedTransaction::from_unsigned_tx(tx).expect("PSBT should work here")),
    //        buy_adaptor_sig: (sig, point, pdleq),
    //    };
    //}

    #[test]
    fn create_buy_procedure_signature_message() {
        let ecdsa_sig = "3045022100b75f569de3e57f4f445bcf9e42be9e5b5128f317ab86e451fdfe7be5ffd6a7da0220776b30307b5d761512635dc0394573be7fe17b5300b160340dae370b641bc4ca";

        let sig =
            Signature::from_der(&hex::decode(ecdsa_sig).expect("HEX decode should work here"))
                .expect("Parse DER should work here");

        let point = PublicKey::from_slice(&[
            0x02, 0xc6, 0x6e, 0x7d, 0x89, 0x66, 0xb5, 0xc5, 0x55, 0xaf, 0x58, 0x05, 0x98, 0x9d,
            0xa9, 0xfb, 0xf8, 0xdb, 0x95, 0xe1, 0x56, 0x31, 0xce, 0x35, 0x8c, 0x3a, 0x17, 0x10,
            0xc9, 0x62, 0x67, 0x90, 0x63,
        ])
        .expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");

        let dleq = PDLEQ;
        
        let adaptor = ECDSAAdaptorSig {
            sig,
            point,
            dleq,
        };

        let msg = TestMsg::<Bitcoin, ECDSAScripts> {
            refund_adaptor_sig: adaptor,
        };

        println!("{:#?}", msg);

        println!("{:?}", lightning_serialize(&msg));
        assert!(false);
    }
}
