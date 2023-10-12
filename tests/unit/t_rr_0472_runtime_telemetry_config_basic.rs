//! Integration test for `RR-0472` (basic).
//! Runtime telemetry config fuzz CLI integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0472_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdd, 0xdf];
    let first = relayring::capabilities::rr_0472_runtime_telemetry_config::evaluate(fixture).expect("RR-0472: Runtime telemetry config fuzz CLI integrate validator v17");
    let second = relayring::capabilities::rr_0472_runtime_telemetry_config::evaluate(fixture).expect("RR-0472: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0472: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0472: window consumes the whole buffer");
}
