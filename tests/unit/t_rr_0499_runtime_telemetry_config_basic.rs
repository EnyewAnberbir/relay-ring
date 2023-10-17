//! Integration test for `RR-0499` (basic).
//! Runtime telemetry config fuzz CLI optimize registry v44 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0499_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0xfa];
    let first = relayring::capabilities::rr_0499_runtime_telemetry_config::evaluate(fixture).expect("RR-0499: Runtime telemetry config fuzz CLI optimize registry v44");
    let second = relayring::capabilities::rr_0499_runtime_telemetry_config::evaluate(fixture).expect("RR-0499: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0499: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0499: scanner should emit domain hints");
}
