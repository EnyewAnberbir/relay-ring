//! Integration test for `RR-0305` (basic).
//! Gateway agent relay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0305_gateway_agent_relay_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let first = relayring::capabilities::rr_0305_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0305: Gateway agent relay validate resolver v5");
    let second = relayring::capabilities::rr_0305_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0305: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0305: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0305: window consumes the whole buffer");
}
