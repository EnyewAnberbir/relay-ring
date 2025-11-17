//! Integration test for `RR-0984` (stream).
//! Extended: Runtime telemetry config fuzz CLI benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0984_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let direct = relayring::capabilities::rr_0984_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0984: direct Extended: Runtime telemetry config fuzz CLI benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0984_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0984: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0984: stream path must consume input");
}
