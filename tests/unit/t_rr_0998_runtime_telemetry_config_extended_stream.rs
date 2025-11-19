//! Integration test for `RR-0998` (stream).
//! Extended: Runtime telemetry config fuzz CLI wire planner v43 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0998_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let direct = relayring::capabilities::rr_0998_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0998: direct Extended: Runtime telemetry config fuzz CLI wire planner v43");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0998_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0998: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0998: stream path must consume input");
}
