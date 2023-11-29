//! Integration test for `RR-0832` (basic).
//! Extended: Gateway agent relay harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0832_gateway_agent_relay_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let first = relayring::capabilities::rr_0832_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0832: Extended: Gateway agent relay harden index v32");
    let second = relayring::capabilities::rr_0832_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0832: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0832: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0832: window consumes the whole buffer");
}
