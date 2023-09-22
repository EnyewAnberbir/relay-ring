//! Integration test for `RR-0331` (basic).
//! Gateway agent relay extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0331_gateway_agent_relay_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x52];
    let first = relayring::capabilities::rr_0331_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0331: Gateway agent relay extend codec v31");
    let second = relayring::capabilities::rr_0331_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0331: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0331: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0331: stats visits every byte");
}
