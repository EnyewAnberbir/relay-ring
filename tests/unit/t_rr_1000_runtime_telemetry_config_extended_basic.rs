//! Integration test for `RR-1000` (basic).
//! Extended: Runtime telemetry config fuzz CLI validate resolver v45 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_1000_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let first = relayring::capabilities::rr_1000_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-1000: Extended: Runtime telemetry config fuzz CLI validate resolver v45");
    let second = relayring::capabilities::rr_1000_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-1000: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-1000: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-1000: window consumes the whole buffer");
}
