//! Integration test for `RR-0456` (stream).
//! Runtime telemetry config fuzz CLI extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0456_runtime_telemetry_config_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let direct = relayring::capabilities::rr_0456_runtime_telemetry_config::evaluate(fixture).expect("RR-0456: direct Runtime telemetry config fuzz CLI extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0456_runtime_telemetry_config::evaluate(&copied).expect("RR-0456: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0456: stream path must consume input");
}
