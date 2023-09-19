//! Integration test for `RR-0301` (basic).
//! Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0301_gateway_agent_relay_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let first = relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0301: Gateway agent relay extend codec v1");
    let second = relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0301: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0301: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0301: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
