//! Integration test for `RR-0812` (basic).
//! Extended: Gateway agent relay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0812_gateway_agent_relay_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let first = relayring::capabilities::rr_0812_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0812: Extended: Gateway agent relay harden index v12");
    let second = relayring::capabilities::rr_0812_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0812: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0812: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0812: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
