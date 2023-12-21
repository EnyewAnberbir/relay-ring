//! Integration test for `RR-0977` (basic).
//! Extended: Runtime telemetry config fuzz CLI harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0977_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let first = relayring::capabilities::rr_0977_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0977: Extended: Runtime telemetry config fuzz CLI harden index v22");
    let second = relayring::capabilities::rr_0977_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0977: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0977: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0977: window consumes the whole buffer");
}
