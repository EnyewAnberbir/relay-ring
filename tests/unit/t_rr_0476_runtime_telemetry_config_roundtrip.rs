//! Integration test for `RR-0476` (roundtrip).
//! Runtime telemetry config fuzz CLI extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0476_runtime_telemetry_config_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let a = relayring::capabilities::rr_0476_runtime_telemetry_config::evaluate(fixture).expect("RR-0476 first pass");
    let b = relayring::capabilities::rr_0476_runtime_telemetry_config::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
