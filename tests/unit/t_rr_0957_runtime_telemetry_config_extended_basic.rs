//! Integration test for `RR-0957` (basic).
//! Extended: Runtime telemetry config fuzz CLI harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0957_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc6, 0xc8];
    let first = relayring::capabilities::rr_0957_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0957: Extended: Runtime telemetry config fuzz CLI harden index v2");
    let second = relayring::capabilities::rr_0957_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0957: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0957: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0957: stats visits every byte");
}
