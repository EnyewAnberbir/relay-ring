//! Integration test for `RR-0328` (basic).
//! Gateway agent relay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0328_gateway_agent_relay_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4d, 0x4f];
    let first = relayring::capabilities::rr_0328_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0328: Gateway agent relay refactor mutator v28");
    let second = relayring::capabilities::rr_0328_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0328: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0328: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0328: stats visits every byte");
}
