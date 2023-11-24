//! Integration test for `RR-0804` (basic).
//! Extended: Gateway agent relay optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0804_gateway_agent_relay_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x2f];
    let first = relayring::capabilities::rr_0804_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0804: Extended: Gateway agent relay optimize registry v4");
    let second = relayring::capabilities::rr_0804_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0804: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0804: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0804: window consumes the whole buffer");
}
