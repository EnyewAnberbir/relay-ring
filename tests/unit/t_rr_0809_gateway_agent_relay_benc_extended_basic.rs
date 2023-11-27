//! Integration test for `RR-0809` (basic).
//! Extended: Gateway agent relay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0809_gateway_agent_relay_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let first = relayring::capabilities::rr_0809_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0809: Extended: Gateway agent relay benchmark reporter v9");
    let second = relayring::capabilities::rr_0809_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0809: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0809: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0809: scanner should emit domain hints");
}
