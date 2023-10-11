//! Integration test for `RR-0462` (basic).
//! Runtime telemetry config fuzz CLI integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0462_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd3, 0xd5];
    let first = relayring::capabilities::rr_0462_runtime_telemetry_config::evaluate(fixture).expect("RR-0462: Runtime telemetry config fuzz CLI integrate validator v7");
    let second = relayring::capabilities::rr_0462_runtime_telemetry_config::evaluate(fixture).expect("RR-0462: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0462: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0462: stats visits every byte");
}
