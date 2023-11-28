//! Integration test for `RR-0821` (basic).
//! Extended: Gateway agent relay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0821_gateway_agent_relay_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40];
    let first = relayring::capabilities::rr_0821_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0821: Extended: Gateway agent relay extend codec v21");
    let second = relayring::capabilities::rr_0821_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0821: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0821: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0821: window consumes the whole buffer");
}
