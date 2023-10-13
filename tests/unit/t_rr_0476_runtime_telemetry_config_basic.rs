//! Integration test for `RR-0476` (basic).
//! Runtime telemetry config fuzz CLI extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0476_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let first = relayring::capabilities::rr_0476_runtime_telemetry_config::evaluate(fixture).expect("RR-0476: Runtime telemetry config fuzz CLI extend codec v21");
    let second = relayring::capabilities::rr_0476_runtime_telemetry_config::evaluate(fixture).expect("RR-0476: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0476: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0476: window consumes the whole buffer");
}
