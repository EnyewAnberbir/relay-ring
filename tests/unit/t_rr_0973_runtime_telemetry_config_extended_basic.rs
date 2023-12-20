//! Integration test for `RR-0973` (basic).
//! Extended: Runtime telemetry config fuzz CLI refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0973_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6, 0xd8];
    let first = relayring::capabilities::rr_0973_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0973: Extended: Runtime telemetry config fuzz CLI refactor mutator v18");
    let second = relayring::capabilities::rr_0973_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0973: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0973: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0973: stats visits every byte");
}
