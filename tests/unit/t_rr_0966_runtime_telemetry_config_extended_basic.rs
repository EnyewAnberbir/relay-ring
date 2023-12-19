//! Integration test for `RR-0966` (basic).
//! Extended: Runtime telemetry config fuzz CLI extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0966_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcf, 0xd1];
    let first = relayring::capabilities::rr_0966_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0966: Extended: Runtime telemetry config fuzz CLI extend codec v11");
    let second = relayring::capabilities::rr_0966_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0966: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0966: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0966: stats visits every byte");
}
