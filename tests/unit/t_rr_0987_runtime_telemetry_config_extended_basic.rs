//! Integration test for `RR-0987` (basic).
//! Extended: Runtime telemetry config fuzz CLI harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0987_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0xe6];
    let first = relayring::capabilities::rr_0987_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0987: Extended: Runtime telemetry config fuzz CLI harden index v32");
    let second = relayring::capabilities::rr_0987_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0987: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0987: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0987: window consumes the whole buffer");
}
