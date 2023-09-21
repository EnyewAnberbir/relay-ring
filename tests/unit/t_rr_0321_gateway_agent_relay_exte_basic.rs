//! Integration test for `RR-0321` (basic).
//! Gateway agent relay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0321_gateway_agent_relay_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let first = relayring::capabilities::rr_0321_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0321: Gateway agent relay extend codec v21");
    let second = relayring::capabilities::rr_0321_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0321: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0321: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0321: stats visits every byte");
}
