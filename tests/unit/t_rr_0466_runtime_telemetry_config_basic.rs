//! Integration test for `RR-0466` (basic).
//! Runtime telemetry config fuzz CLI extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0466_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0xd9];
    let first = relayring::capabilities::rr_0466_runtime_telemetry_config::evaluate(fixture).expect("RR-0466: Runtime telemetry config fuzz CLI extend codec v11");
    let second = relayring::capabilities::rr_0466_runtime_telemetry_config::evaluate(fixture).expect("RR-0466: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0466: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0466: window consumes the whole buffer");
}
