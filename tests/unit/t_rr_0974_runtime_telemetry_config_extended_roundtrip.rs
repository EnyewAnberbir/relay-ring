//! Integration test for `RR-0974` (roundtrip).
//! Extended: Runtime telemetry config fuzz CLI benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0974_runtime_telemetry_config_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0xd9];
    let a = relayring::capabilities::rr_0974_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0974 first pass");
    let b = relayring::capabilities::rr_0974_runtime_telemetry_config_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
