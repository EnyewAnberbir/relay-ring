//! Integration test for `RR-0802` (basic).
//! Extended: Gateway agent relay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0802_gateway_agent_relay_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let first = relayring::capabilities::rr_0802_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0802: Extended: Gateway agent relay harden index v2");
    let second = relayring::capabilities::rr_0802_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0802: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0802: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0802: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
