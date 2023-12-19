//! Integration test for `RR-0970` (basic).
//! Extended: Runtime telemetry config fuzz CLI validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0970_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd3, 0xd5];
    let first = relayring::capabilities::rr_0970_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0970: Extended: Runtime telemetry config fuzz CLI validate resolver v15");
    let second = relayring::capabilities::rr_0970_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0970: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0970: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0970: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
