//! Integration test for `RR-0815` (basic).
//! Extended: Gateway agent relay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0815_gateway_agent_relay_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let first = relayring::capabilities::rr_0815_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0815: Extended: Gateway agent relay validate resolver v15");
    let second = relayring::capabilities::rr_0815_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0815: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0815: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0815: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
