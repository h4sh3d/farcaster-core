use farcaster_chains::bitcoin::{Amount, Bitcoin, CSVTimelock, SatPerVByte};
use farcaster_chains::monero::Monero;

use farcaster_core::blockchain::{Blockchain, FeeStrategy, Network};
use farcaster_core::consensus::{self, deserialize, serialize_hex};
use farcaster_core::negotiation::{Buy, Offer, PublicOffer, Sell};
use farcaster_core::role::SwapRole;

use internet2::{RemoteSocketAddr, RemoteNodeAddr};

use std::str::FromStr;

#[test]
fn create_offer() {
    let hex = "020000008080000080080500000000000000080600000000000000040700000004080000000108090000000000000002";
    let offer = Offer {
        network: Network::Testnet,
        arbitrating: Bitcoin::new(),
        accordant: Monero::new(),
        arbitrating_assets: Amount::from_sat(5),
        accordant_assets: 6,
        cancel_timelock: CSVTimelock::new(7),
        punish_timelock: CSVTimelock::new(8),
        fee_strategy: FeeStrategy::Fixed(SatPerVByte::from_sat(9)),
        maker_role: SwapRole::Bob,
    };

    assert_eq!(hex, serialize_hex(&offer));
}

#[test]
fn maker_buy_arbitrating_assets_offer() {
    let offer = Buy::some(Bitcoin::new(), Amount::from_sat(100000))
        .with(Monero::new(), 200)
        .with_timelocks(CSVTimelock::new(10), CSVTimelock::new(10))
        .with_fee(FeeStrategy::Fixed(SatPerVByte::from_sat(20)))
        .on(Network::Testnet)
        .to_offer();
    assert!(offer.is_some());
    assert_eq!(offer.expect("an offer").maker_role, SwapRole::Alice);
}

#[test]
fn maker_sell_arbitrating_assets_offer() {
    let offer = Sell::some(Bitcoin::new(), Amount::from_sat(100000))
        .for_some(Monero::new(), 200)
        .with_timelocks(CSVTimelock::new(10), CSVTimelock::new(10))
        .with_fee(FeeStrategy::Fixed(SatPerVByte::from_sat(20)))
        .on(Network::Testnet)
        .to_offer();
    assert!(offer.is_some());
    assert_eq!(offer.expect("an offer").maker_role, SwapRole::Bob);
}

#[test]
fn serialize_public_offer() {
    let hex = "464353574150010002000000808000008008a08601000000000008c800000000000000040a000000040a000000010814000000000000000203b31a0a70343bb46f3db3768296ac5027f9873921b37f852860c690063ff9e4c90000000000000000000000000000000000000000000000000000000000000000000000260700";
    let offer = Sell::some(Bitcoin::new(), Amount::from_sat(100000))
        .for_some(Monero::new(), 200)
        .with_timelocks(CSVTimelock::new(10), CSVTimelock::new(10))
        .with_fee(FeeStrategy::Fixed(SatPerVByte::from_sat(20)))
        .on(Network::Testnet)
        .to_offer()
        .unwrap();
    let overlay = FromStr::from_str("tcp").unwrap();
    let ip = FromStr::from_str("0.0.0.0").unwrap();
    let port = FromStr::from_str("9735").unwrap();
    let remote_addr =
        RemoteSocketAddr::with_ip_addr(overlay, ip, port);

    let secp = secp256k1::Secp256k1::new();
    let sk =
        bitcoin::PrivateKey::from_wif("L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D").unwrap().key;
    let node_id = secp256k1::PublicKey::from_secret_key(&secp, &sk);
    let peer = RemoteNodeAddr {
        node_id,
        remote_addr,
    };
    let public_offer = offer.to_public_v1(peer);

    assert_eq!(hex, serialize_hex(&public_offer));
}



#[test]
fn check_public_offer_magic_bytes() {

    let valid = "4643535741500100020000008080000080086400000000000000086400000000000000040a000000041e0000000108140000000000000001026981c0e141351c1aae13014379d629dfddb3b5375c1265c34203b5d13c69cd270000000000000000000000000000000000000000000000000000000000000000000000260700";
    let pub_offer: Result<PublicOffer<Bitcoin, Monero>, consensus::Error> =
        deserialize(&hex::decode(valid).unwrap()[..]);
    assert!(pub_offer.is_ok());

    let invalid = "474353574150010002000000808000008008a08601000000000008c800000000000000040a000000040a0000000108140000000000000002";
    let pub_offer: Result<PublicOffer<Bitcoin, Monero>, consensus::Error> =
        deserialize(&hex::decode(invalid).unwrap()[..]);
    assert!(pub_offer.is_err());
}