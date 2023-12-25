//! Integration test for `RR-0992` (basic).
//! Extended: Runtime telemetry config fuzz CLI integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0992_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let first = relayring::capabilities::rr_0992_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0992: Extended: Runtime telemetry config fuzz CLI integrate validator v37");
    let second = relayring::capabilities::rr_0992_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0992: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0992: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0992: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
