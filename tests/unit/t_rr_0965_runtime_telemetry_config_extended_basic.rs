//! Integration test for `RR-0965` (basic).
//! Extended: Runtime telemetry config fuzz CLI implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0965_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xce, 0xd0];
    let first = relayring::capabilities::rr_0965_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0965: Extended: Runtime telemetry config fuzz CLI implement pipeline v10");
    let second = relayring::capabilities::rr_0965_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0965: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0965: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0965: scanner should emit domain hints");
}
