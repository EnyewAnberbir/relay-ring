//! Integration test for `RR-0493` (basic).
//! Runtime telemetry config fuzz CLI refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0493_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf2, 0xf4];
    let first = relayring::capabilities::rr_0493_runtime_telemetry_config::evaluate(fixture).expect("RR-0493: Runtime telemetry config fuzz CLI refactor mutator v38");
    let second = relayring::capabilities::rr_0493_runtime_telemetry_config::evaluate(fixture).expect("RR-0493: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0493: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0493: window consumes the whole buffer");
}
