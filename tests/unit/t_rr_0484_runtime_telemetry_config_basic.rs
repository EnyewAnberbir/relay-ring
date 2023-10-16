//! Integration test for `RR-0484` (basic).
//! Runtime telemetry config fuzz CLI benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0484_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let first = relayring::capabilities::rr_0484_runtime_telemetry_config::evaluate(fixture).expect("RR-0484: Runtime telemetry config fuzz CLI benchmark reporter v29");
    let second = relayring::capabilities::rr_0484_runtime_telemetry_config::evaluate(fixture).expect("RR-0484: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0484: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0484: stats visits every byte");
}
