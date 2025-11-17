//! Integration test for `RR-0975` (stream).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0975_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let direct = relayring::capabilities::rr_0975_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0975: direct Extended: Runtime telemetry config fuzz CLI implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0975_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0975: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0975: stream path must consume input");
}
