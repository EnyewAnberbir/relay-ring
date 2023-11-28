//! Integration test for `RR-0827` (basic).
//! Extended: Gateway agent relay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0827_gateway_agent_relay_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let first = relayring::capabilities::rr_0827_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0827: Extended: Gateway agent relay integrate validator v27");
    let second = relayring::capabilities::rr_0827_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0827: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0827: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0827: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
