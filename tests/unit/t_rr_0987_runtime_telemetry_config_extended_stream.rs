//! Integration test for `RR-0987` (stream).
//! Extended: Runtime telemetry config fuzz CLI harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0987_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0xe6];
    let direct = relayring::capabilities::rr_0987_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0987: direct Extended: Runtime telemetry config fuzz CLI harden index v32");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0987_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0987: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0987: stream path must consume input");
}
