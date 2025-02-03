//! Integration test for `RR-0965` (roundtrip).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0965_runtime_telemetry_config_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let a = relayring::capabilities::rr_0965_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0965 first pass");
    let b = relayring::capabilities::rr_0965_runtime_telemetry_config_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
