//! Integration test for `RR-0979` (basic).
//! Extended: Runtime telemetry config fuzz CLI optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0979_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdc, 0xde];
    let first = relayring::capabilities::rr_0979_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0979: Extended: Runtime telemetry config fuzz CLI optimize registry v24");
    let second = relayring::capabilities::rr_0979_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0979: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0979: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0979: stats visits every byte");
}
