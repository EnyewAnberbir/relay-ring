//! Integration test for `RR-0971` (stream).
//! Extended: Runtime telemetry config fuzz CLI export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0971_runtime_telemetry_config_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0xd6];
    let direct = relayring::capabilities::rr_0971_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0971: direct Extended: Runtime telemetry config fuzz CLI export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0971_runtime_telemetry_config_extended::evaluate(&copied).expect("RR-0971: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0971: stream path must consume input");
}
