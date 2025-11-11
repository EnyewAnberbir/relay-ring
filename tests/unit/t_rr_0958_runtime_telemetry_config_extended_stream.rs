//! Integration test for `RR-0958` (stream).
//! Extended: Runtime telemetry config fuzz CLI wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0958_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc7, 0xc9];
    let direct = relayring::capabilities::rr_0958_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0958: direct Extended: Runtime telemetry config fuzz CLI wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0958_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0958: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0958: stream path must consume input");
}
