//! Integration test for `RR-0805` (basic).
//! Extended: Gateway agent relay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0805_gateway_agent_relay_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let first = relayring::capabilities::rr_0805_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0805: Extended: Gateway agent relay validate resolver v5");
    let second = relayring::capabilities::rr_0805_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0805: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0805: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0805: window consumes the whole buffer");
}
