//! Integration test for `RR-0969` (basic).
//! Extended: Runtime telemetry config fuzz CLI optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0969_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd2, 0xd4];
    let first = relayring::capabilities::rr_0969_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0969: Extended: Runtime telemetry config fuzz CLI optimize registry v14");
    let second = relayring::capabilities::rr_0969_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0969: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0969: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0969: window consumes the whole buffer");
}
