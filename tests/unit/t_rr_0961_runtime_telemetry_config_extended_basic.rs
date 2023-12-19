//! Integration test for `RR-0961` (basic).
//! Extended: Runtime telemetry config fuzz CLI export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0961_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xca, 0xcc];
    let first = relayring::capabilities::rr_0961_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0961: Extended: Runtime telemetry config fuzz CLI export adapter v6");
    let second = relayring::capabilities::rr_0961_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0961: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0961: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0961: window consumes the whole buffer");
}
