//! Integration test for `RR-0967` (basic).
//! Extended: Runtime telemetry config fuzz CLI harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0967_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0xd2];
    let first = relayring::capabilities::rr_0967_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0967: Extended: Runtime telemetry config fuzz CLI harden index v12");
    let second = relayring::capabilities::rr_0967_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0967: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0967: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0967: stats visits every byte");
}
