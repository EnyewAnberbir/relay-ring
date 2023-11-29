//! Integration test for `RR-0835` (basic).
//! Extended: Gateway agent relay validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0835_gateway_agent_relay_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x4e];
    let first = relayring::capabilities::rr_0835_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0835: Extended: Gateway agent relay validate resolver v35");
    let second = relayring::capabilities::rr_0835_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0835: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0835: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0835: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
