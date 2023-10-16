//! Integration test for `RR-0495` (basic).
//! Runtime telemetry config fuzz CLI implement pipeline v40 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0495_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4, 0xf6];
    let first = relayring::capabilities::rr_0495_runtime_telemetry_config::evaluate(fixture).expect("RR-0495: Runtime telemetry config fuzz CLI implement pipeline v40");
    let second = relayring::capabilities::rr_0495_runtime_telemetry_config::evaluate(fixture).expect("RR-0495: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0495: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0495: stats visits every byte");
}
