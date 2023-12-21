//! Integration test for `RR-0975` (basic).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0975_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let first = relayring::capabilities::rr_0975_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0975: Extended: Runtime telemetry config fuzz CLI implement pipeline v20");
    let second = relayring::capabilities::rr_0975_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0975: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0975: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0975: stats visits every byte");
}
