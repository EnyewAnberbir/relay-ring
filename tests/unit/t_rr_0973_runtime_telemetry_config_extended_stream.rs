//! Integration test for `RR-0973` (stream).
//! Extended: Runtime telemetry config fuzz CLI refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0973_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6, 0xd8];
    let direct = relayring::capabilities::rr_0973_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0973: direct Extended: Runtime telemetry config fuzz CLI refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0973_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0973: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0973: stream path must consume input");
}
