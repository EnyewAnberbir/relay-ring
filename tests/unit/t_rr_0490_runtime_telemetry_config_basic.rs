//! Integration test for `RR-0490` (basic).
//! Runtime telemetry config fuzz CLI validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0490_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let first = relayring::capabilities::rr_0490_runtime_telemetry_config::evaluate(fixture).expect("RR-0490: Runtime telemetry config fuzz CLI validate resolver v35");
    let second = relayring::capabilities::rr_0490_runtime_telemetry_config::evaluate(fixture).expect("RR-0490: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0490: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0490: window consumes the whole buffer");
}
