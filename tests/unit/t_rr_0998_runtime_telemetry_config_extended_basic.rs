//! Integration test for `RR-0998` (basic).
//! Extended: Runtime telemetry config fuzz CLI wire planner v43 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0998_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let first = relayring::capabilities::rr_0998_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0998: Extended: Runtime telemetry config fuzz CLI wire planner v43");
    let second = relayring::capabilities::rr_0998_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0998: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0998: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0998: scanner should emit domain hints");
}
