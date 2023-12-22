//! Integration test for `RR-0981` (basic).
//! Extended: Runtime telemetry config fuzz CLI export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0981_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xde, 0xe0];
    let first = relayring::capabilities::rr_0981_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0981: Extended: Runtime telemetry config fuzz CLI export adapter v26");
    let second = relayring::capabilities::rr_0981_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0981: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0981: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0981: stats visits every byte");
}
