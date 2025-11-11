//! Integration test for `RR-0956` (stream).
//! Extended: Runtime telemetry config fuzz CLI extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0956_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc5, 0xc7];
    let direct = relayring::capabilities::rr_0956_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0956: direct Extended: Runtime telemetry config fuzz CLI extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0956_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0956: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0956: stream path must consume input");
}
