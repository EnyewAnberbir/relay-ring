//! Integration test for `RR-0985` (roundtrip).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0985_runtime_telemetry_config_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe2, 0xe4];
    let a = relayring::capabilities::rr_0985_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0985 first pass");
    let b = relayring::capabilities::rr_0985_runtime_telemetry_config_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
