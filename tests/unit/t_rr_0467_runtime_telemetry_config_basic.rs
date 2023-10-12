//! Integration test for `RR-0467` (basic).
//! Runtime telemetry config fuzz CLI harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0467_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let first = relayring::capabilities::rr_0467_runtime_telemetry_config::evaluate(fixture).expect("RR-0467: Runtime telemetry config fuzz CLI harden index v12");
    let second = relayring::capabilities::rr_0467_runtime_telemetry_config::evaluate(fixture).expect("RR-0467: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0467: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0467: window consumes the whole buffer");
}
