//! Integration test for `RR-0470` (basic).
//! Runtime telemetry config fuzz CLI validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0470_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdb, 0xdd];
    let first = relayring::capabilities::rr_0470_runtime_telemetry_config::evaluate(fixture).expect("RR-0470: Runtime telemetry config fuzz CLI validate resolver v15");
    let second = relayring::capabilities::rr_0470_runtime_telemetry_config::evaluate(fixture).expect("RR-0470: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0470: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0470: stats visits every byte");
}
