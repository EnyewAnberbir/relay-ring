//! Integration test for `RR-0982` (basic).
//! Extended: Runtime telemetry config fuzz CLI integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0982_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0xe1];
    let first = relayring::capabilities::rr_0982_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0982: Extended: Runtime telemetry config fuzz CLI integrate validator v27");
    let second = relayring::capabilities::rr_0982_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0982: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0982: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0982: window consumes the whole buffer");
}
