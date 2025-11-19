//! Integration test for `RR-1000` (stream).
//! Extended: Runtime telemetry config fuzz CLI validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_1000_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let direct = relayring::capabilities::rr_1000_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-1000: direct Extended: Runtime telemetry config fuzz CLI validate resolver v45");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_1000_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-1000: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-1000: stream path must consume input");
}
