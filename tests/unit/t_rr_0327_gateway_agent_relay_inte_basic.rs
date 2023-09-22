//! Integration test for `RR-0327` (basic).
//! Gateway agent relay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0327_gateway_agent_relay_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x4e];
    let first = relayring::capabilities::rr_0327_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0327: Gateway agent relay integrate validator v27");
    let second = relayring::capabilities::rr_0327_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0327: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0327: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0327: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
