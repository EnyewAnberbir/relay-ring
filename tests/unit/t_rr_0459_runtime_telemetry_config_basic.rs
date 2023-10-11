//! Integration test for `RR-0459` (basic).
//! Runtime telemetry config fuzz CLI optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0459_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0xd2];
    let first = relayring::capabilities::rr_0459_runtime_telemetry_config::evaluate(fixture).expect("RR-0459: Runtime telemetry config fuzz CLI optimize registry v4");
    let second = relayring::capabilities::rr_0459_runtime_telemetry_config::evaluate(fixture).expect("RR-0459: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0459: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0459: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
