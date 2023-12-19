//! Integration test for `RR-0968` (basic).
//! Extended: Runtime telemetry config fuzz CLI wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0968_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let first = relayring::capabilities::rr_0968_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0968: Extended: Runtime telemetry config fuzz CLI wire planner v13");
    let second = relayring::capabilities::rr_0968_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0968: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0968: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0968: stats visits every byte");
}
