//! Integration test for `RR-0478` (basic).
//! Runtime telemetry config fuzz CLI wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0478_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe3, 0xe5];
    let first = relayring::capabilities::rr_0478_runtime_telemetry_config::evaluate(fixture).expect("RR-0478: Runtime telemetry config fuzz CLI wire planner v23");
    let second = relayring::capabilities::rr_0478_runtime_telemetry_config::evaluate(fixture).expect("RR-0478: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0478: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0478: window consumes the whole buffer");
}
