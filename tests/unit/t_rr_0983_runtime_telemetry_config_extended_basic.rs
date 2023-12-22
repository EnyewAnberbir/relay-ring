//! Integration test for `RR-0983` (basic).
//! Extended: Runtime telemetry config fuzz CLI refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0983_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let first = relayring::capabilities::rr_0983_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0983: Extended: Runtime telemetry config fuzz CLI refactor mutator v28");
    let second = relayring::capabilities::rr_0983_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0983: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0983: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0983: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
