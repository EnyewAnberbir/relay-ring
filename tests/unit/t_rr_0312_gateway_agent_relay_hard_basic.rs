//! Integration test for `RR-0312` (basic).
//! Gateway agent relay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0312_gateway_agent_relay_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let first = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0312: Gateway agent relay harden index v12");
    let second = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0312: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0312: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0312: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
