//! Integration test for `RR-0318` (basic).
//! Gateway agent relay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0318_gateway_agent_relay_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let first = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0318: Gateway agent relay refactor mutator v18");
    let second = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0318: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0318: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0318: stats visits every byte");
}
