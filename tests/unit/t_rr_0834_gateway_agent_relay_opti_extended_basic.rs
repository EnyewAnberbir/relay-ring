//! Integration test for `RR-0834` (basic).
//! Extended: Gateway agent relay optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0834_gateway_agent_relay_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x4d];
    let first = relayring::capabilities::rr_0834_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0834: Extended: Gateway agent relay optimize registry v34");
    let second = relayring::capabilities::rr_0834_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0834: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0834: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0834: window consumes the whole buffer");
}
