//! Integration test for `RR-0475` (basic).
//! Runtime telemetry config fuzz CLI implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0475_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let first = relayring::capabilities::rr_0475_runtime_telemetry_config::evaluate(fixture).expect("RR-0475: Runtime telemetry config fuzz CLI implement pipeline v20");
    let second = relayring::capabilities::rr_0475_runtime_telemetry_config::evaluate(fixture).expect("RR-0475: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0475: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0475: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
