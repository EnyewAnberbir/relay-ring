//! Integration test for `RR-0469` (basic).
//! Runtime telemetry config fuzz CLI optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0469_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let first = relayring::capabilities::rr_0469_runtime_telemetry_config::evaluate(fixture).expect("RR-0469: Runtime telemetry config fuzz CLI optimize registry v14");
    let second = relayring::capabilities::rr_0469_runtime_telemetry_config::evaluate(fixture).expect("RR-0469: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0469: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0469: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
