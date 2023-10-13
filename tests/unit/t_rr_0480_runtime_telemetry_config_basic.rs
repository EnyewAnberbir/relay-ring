//! Integration test for `RR-0480` (basic).
//! Runtime telemetry config fuzz CLI validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0480_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe5, 0xe7];
    let first = relayring::capabilities::rr_0480_runtime_telemetry_config::evaluate(fixture).expect("RR-0480: Runtime telemetry config fuzz CLI validate resolver v25");
    let second = relayring::capabilities::rr_0480_runtime_telemetry_config::evaluate(fixture).expect("RR-0480: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0480: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0480: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
