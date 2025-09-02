//! Integration test for `RR-0499` (stream).
//! Runtime telemetry config fuzz CLI optimize registry v44 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0499_runtime_telemetry_config_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0xfa];
    let direct = relayring::capabilities::rr_0499_runtime_telemetry_config::evaluate(fixture).expect("RR-0499: direct Runtime telemetry config fuzz CLI optimize registry v44");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0499_runtime_telemetry_config::evaluate(&copied).expect("RR-0499: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0499: stream path must consume input");
}
