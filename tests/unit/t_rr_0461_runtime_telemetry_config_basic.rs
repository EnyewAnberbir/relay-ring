//! Integration test for `RR-0461` (basic).
//! Runtime telemetry config fuzz CLI export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0461_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd2, 0xd4];
    let first = relayring::capabilities::rr_0461_runtime_telemetry_config::evaluate(fixture).expect("RR-0461: Runtime telemetry config fuzz CLI export adapter v6");
    let second = relayring::capabilities::rr_0461_runtime_telemetry_config::evaluate(fixture).expect("RR-0461: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0461: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0461: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
