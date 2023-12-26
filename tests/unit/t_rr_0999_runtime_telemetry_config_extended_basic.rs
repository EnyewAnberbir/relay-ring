//! Integration test for `RR-0999` (basic).
//! Extended: Runtime telemetry config fuzz CLI optimize registry v44 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0999_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xf2];
    let first = relayring::capabilities::rr_0999_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0999: Extended: Runtime telemetry config fuzz CLI optimize registry v44");
    let second = relayring::capabilities::rr_0999_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0999: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0999: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0999: window consumes the whole buffer");
}
