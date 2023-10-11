//! Integration test for `RR-0460` (basic).
//! Runtime telemetry config fuzz CLI validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0460_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let first = relayring::capabilities::rr_0460_runtime_telemetry_config::evaluate(fixture).expect("RR-0460: Runtime telemetry config fuzz CLI validate resolver v5");
    let second = relayring::capabilities::rr_0460_runtime_telemetry_config::evaluate(fixture).expect("RR-0460: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0460: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0460: scanner should emit domain hints");
}
