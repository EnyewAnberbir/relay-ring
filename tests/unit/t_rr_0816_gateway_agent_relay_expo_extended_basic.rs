//! Integration test for `RR-0816` (basic).
//! Extended: Gateway agent relay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0816_gateway_agent_relay_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let first = relayring::capabilities::rr_0816_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0816: Extended: Gateway agent relay export adapter v16");
    let second = relayring::capabilities::rr_0816_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0816: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0816: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0816: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
