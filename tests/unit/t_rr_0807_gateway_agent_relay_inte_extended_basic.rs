//! Integration test for `RR-0807` (basic).
//! Extended: Gateway agent relay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0807_gateway_agent_relay_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let first = relayring::capabilities::rr_0807_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0807: Extended: Gateway agent relay integrate validator v7");
    let second = relayring::capabilities::rr_0807_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0807: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0807: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0807: window consumes the whole buffer");
}
