//! Integration test for `RR-0322` (basic).
//! Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0322_gateway_agent_relay_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let first = relayring::capabilities::rr_0322_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0322: Gateway agent relay harden index v22");
    let second = relayring::capabilities::rr_0322_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0322: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0322: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0322: window consumes the whole buffer");
}
