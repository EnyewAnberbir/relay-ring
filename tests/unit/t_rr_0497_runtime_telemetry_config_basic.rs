//! Integration test for `RR-0497` (basic).
//! Runtime telemetry config fuzz CLI harden index v42 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0497_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0xf8];
    let first = relayring::capabilities::rr_0497_runtime_telemetry_config::evaluate(fixture).expect("RR-0497: Runtime telemetry config fuzz CLI harden index v42");
    let second = relayring::capabilities::rr_0497_runtime_telemetry_config::evaluate(fixture).expect("RR-0497: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0497: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0497: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
