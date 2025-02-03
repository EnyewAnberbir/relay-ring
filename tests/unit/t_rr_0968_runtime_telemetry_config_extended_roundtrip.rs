//! Integration test for `RR-0968` (roundtrip).
//! Extended: Runtime telemetry config fuzz CLI wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0968_runtime_telemetry_config_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let a = relayring::capabilities::rr_0968_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0968 first pass");
    let b = relayring::capabilities::rr_0968_runtime_telemetry_config_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
