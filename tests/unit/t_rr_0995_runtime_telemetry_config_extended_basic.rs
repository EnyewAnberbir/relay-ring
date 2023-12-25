//! Integration test for `RR-0995` (basic).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0995_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xec, 0xee];
    let first = relayring::capabilities::rr_0995_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0995: Extended: Runtime telemetry config fuzz CLI implement pipeline v40");
    let second = relayring::capabilities::rr_0995_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0995: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0995: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0995: window consumes the whole buffer");
}
