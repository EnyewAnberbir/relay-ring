//! Integration test for `RR-0479` (basic).
//! Runtime telemetry config fuzz CLI optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0479_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0xe6];
    let first = relayring::capabilities::rr_0479_runtime_telemetry_config::evaluate(fixture).expect("RR-0479: Runtime telemetry config fuzz CLI optimize registry v24");
    let second = relayring::capabilities::rr_0479_runtime_telemetry_config::evaluate(fixture).expect("RR-0479: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0479: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0479: window consumes the whole buffer");
}
