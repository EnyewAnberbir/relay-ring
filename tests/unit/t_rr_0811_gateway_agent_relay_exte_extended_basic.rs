//! Integration test for `RR-0811` (basic).
//! Extended: Gateway agent relay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0811_gateway_agent_relay_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let first = relayring::capabilities::rr_0811_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0811: Extended: Gateway agent relay extend codec v11");
    let second = relayring::capabilities::rr_0811_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0811: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0811: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0811: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
