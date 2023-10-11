//! Integration test for `RR-0456` (basic).
//! Runtime telemetry config fuzz CLI extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0456_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let first = relayring::capabilities::rr_0456_runtime_telemetry_config::evaluate(fixture).expect("RR-0456: Runtime telemetry config fuzz CLI extend codec v1");
    let second = relayring::capabilities::rr_0456_runtime_telemetry_config::evaluate(fixture).expect("RR-0456: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0456: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0456: stats visits every byte");
}
