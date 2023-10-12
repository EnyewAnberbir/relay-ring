//! Integration test for `RR-0468` (basic).
//! Runtime telemetry config fuzz CLI wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0468_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let first = relayring::capabilities::rr_0468_runtime_telemetry_config::evaluate(fixture).expect("RR-0468: Runtime telemetry config fuzz CLI wire planner v13");
    let second = relayring::capabilities::rr_0468_runtime_telemetry_config::evaluate(fixture).expect("RR-0468: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0468: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0468: scanner should emit domain hints");
}
