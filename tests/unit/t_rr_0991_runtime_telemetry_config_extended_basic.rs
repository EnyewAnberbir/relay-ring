//! Integration test for `RR-0991` (basic).
//! Extended: Runtime telemetry config fuzz CLI export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0991_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8, 0xea];
    let first = relayring::capabilities::rr_0991_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0991: Extended: Runtime telemetry config fuzz CLI export adapter v36");
    let second = relayring::capabilities::rr_0991_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0991: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0991: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0991: scanner should emit domain hints");
}
