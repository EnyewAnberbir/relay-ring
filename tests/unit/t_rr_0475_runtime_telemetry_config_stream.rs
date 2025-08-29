//! Integration test for `RR-0475` (stream).
//! Runtime telemetry config fuzz CLI implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0475_runtime_telemetry_config_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let direct = relayring::capabilities::rr_0475_runtime_telemetry_config::evaluate(fixture).expect("RR-0475: direct Runtime telemetry config fuzz CLI implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0475_runtime_telemetry_config::evaluate(&copied).expect("RR-0475: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0475: stream path must consume input");
}
