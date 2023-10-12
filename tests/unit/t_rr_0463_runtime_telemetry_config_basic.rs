//! Integration test for `RR-0463` (basic).
//! Runtime telemetry config fuzz CLI refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0463_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0xd6];
    let first = relayring::capabilities::rr_0463_runtime_telemetry_config::evaluate(fixture).expect("RR-0463: Runtime telemetry config fuzz CLI refactor mutator v8");
    let second = relayring::capabilities::rr_0463_runtime_telemetry_config::evaluate(fixture).expect("RR-0463: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0463: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0463: window consumes the whole buffer");
}
