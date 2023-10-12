//! Integration test for `RR-0473` (basic).
//! Runtime telemetry config fuzz CLI refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0473_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xde, 0xe0];
    let first = relayring::capabilities::rr_0473_runtime_telemetry_config::evaluate(fixture).expect("RR-0473: Runtime telemetry config fuzz CLI refactor mutator v18");
    let second = relayring::capabilities::rr_0473_runtime_telemetry_config::evaluate(fixture).expect("RR-0473: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0473: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0473: stats visits every byte");
}
