//! Integration test for `RR-0500` (basic).
//! Runtime telemetry config fuzz CLI validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0500_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0xfb];
    let first = relayring::capabilities::rr_0500_runtime_telemetry_config::evaluate(fixture).expect("RR-0500: Runtime telemetry config fuzz CLI validate resolver v45");
    let second = relayring::capabilities::rr_0500_runtime_telemetry_config::evaluate(fixture).expect("RR-0500: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0500: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0500: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
