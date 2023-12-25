//! Integration test for `RR-0990` (basic).
//! Extended: Runtime telemetry config fuzz CLI validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0990_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0xe9];
    let first = relayring::capabilities::rr_0990_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0990: Extended: Runtime telemetry config fuzz CLI validate resolver v35");
    let second = relayring::capabilities::rr_0990_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0990: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0990: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0990: stats visits every byte");
}
