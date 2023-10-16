//! Integration test for `RR-0494` (basic).
//! Runtime telemetry config fuzz CLI benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0494_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf3, 0xf5];
    let first = relayring::capabilities::rr_0494_runtime_telemetry_config::evaluate(fixture).expect("RR-0494: Runtime telemetry config fuzz CLI benchmark reporter v39");
    let second = relayring::capabilities::rr_0494_runtime_telemetry_config::evaluate(fixture).expect("RR-0494: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0494: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0494: window consumes the whole buffer");
}
