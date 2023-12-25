//! Integration test for `RR-0993` (basic).
//! Extended: Runtime telemetry config fuzz CLI refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0993_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xea, 0xec];
    let first = relayring::capabilities::rr_0993_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0993: Extended: Runtime telemetry config fuzz CLI refactor mutator v38");
    let second = relayring::capabilities::rr_0993_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0993: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0993: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0993: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
