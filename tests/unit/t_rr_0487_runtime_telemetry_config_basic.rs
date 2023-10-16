//! Integration test for `RR-0487` (basic).
//! Runtime telemetry config fuzz CLI harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0487_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xec, 0xee];
    let first = relayring::capabilities::rr_0487_runtime_telemetry_config::evaluate(fixture).expect("RR-0487: Runtime telemetry config fuzz CLI harden index v32");
    let second = relayring::capabilities::rr_0487_runtime_telemetry_config::evaluate(fixture).expect("RR-0487: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0487: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0487: scanner should emit domain hints");
}
