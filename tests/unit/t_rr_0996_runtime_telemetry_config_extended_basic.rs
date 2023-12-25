//! Integration test for `RR-0996` (basic).
//! Extended: Runtime telemetry config fuzz CLI extend codec v41 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0996_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xed, 0xef];
    let first = relayring::capabilities::rr_0996_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0996: Extended: Runtime telemetry config fuzz CLI extend codec v41");
    let second = relayring::capabilities::rr_0996_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0996: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0996: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0996: window consumes the whole buffer");
}
