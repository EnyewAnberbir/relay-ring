//! Integration test for `RR-0307` (basic).
//! Gateway agent relay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0307_gateway_agent_relay_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let first = relayring::capabilities::rr_0307_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0307: Gateway agent relay integrate validator v7");
    let second = relayring::capabilities::rr_0307_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0307: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0307: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0307: window consumes the whole buffer");
}
