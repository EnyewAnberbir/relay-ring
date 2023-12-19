//! Integration test for `RR-0971` (basic).
//! Extended: Runtime telemetry config fuzz CLI export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0971_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0xd6];
    let first = relayring::capabilities::rr_0971_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0971: Extended: Runtime telemetry config fuzz CLI export adapter v16");
    let second = relayring::capabilities::rr_0971_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0971: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0971: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0971: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
