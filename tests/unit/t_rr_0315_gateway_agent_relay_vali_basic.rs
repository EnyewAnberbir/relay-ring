//! Integration test for `RR-0315` (basic).
//! Gateway agent relay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0315_gateway_agent_relay_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let first = relayring::capabilities::rr_0315_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0315: Gateway agent relay validate resolver v15");
    let second = relayring::capabilities::rr_0315_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0315: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0315: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0315: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
