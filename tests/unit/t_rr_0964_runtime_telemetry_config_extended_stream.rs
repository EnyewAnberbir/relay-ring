//! Integration test for `RR-0964` (stream).
//! Extended: Runtime telemetry config fuzz CLI benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0964_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let direct = relayring::capabilities::rr_0964_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0964: direct Extended: Runtime telemetry config fuzz CLI benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0964_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0964: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0964: stream path must consume input");
}
