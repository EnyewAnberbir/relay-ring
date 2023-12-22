//! Integration test for `RR-0984` (basic).
//! Extended: Runtime telemetry config fuzz CLI benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0984_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let first = relayring::capabilities::rr_0984_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0984: Extended: Runtime telemetry config fuzz CLI benchmark reporter v29");
    let second = relayring::capabilities::rr_0984_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0984: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0984: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0984: scanner should emit domain hints");
}
