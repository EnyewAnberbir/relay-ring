//! Integration test for `RR-0498` (basic).
//! Runtime telemetry config fuzz CLI wire planner v43 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0498_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf7, 0xf9];
    let first = relayring::capabilities::rr_0498_runtime_telemetry_config::evaluate(fixture).expect("RR-0498: Runtime telemetry config fuzz CLI wire planner v43");
    let second = relayring::capabilities::rr_0498_runtime_telemetry_config::evaluate(fixture).expect("RR-0498: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0498: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0498: scanner should emit domain hints");
}
