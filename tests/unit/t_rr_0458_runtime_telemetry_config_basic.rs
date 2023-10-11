//! Integration test for `RR-0458` (basic).
//! Runtime telemetry config fuzz CLI wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0458_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0xd1];
    let first = relayring::capabilities::rr_0458_runtime_telemetry_config::evaluate(fixture).expect("RR-0458: Runtime telemetry config fuzz CLI wire planner v3");
    let second = relayring::capabilities::rr_0458_runtime_telemetry_config::evaluate(fixture).expect("RR-0458: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0458: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0458: stats visits every byte");
}
