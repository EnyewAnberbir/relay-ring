//! Integration test for `RR-0959` (basic).
//! Extended: Runtime telemetry config fuzz CLI optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0959_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let first = relayring::capabilities::rr_0959_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0959: Extended: Runtime telemetry config fuzz CLI optimize registry v4");
    let second = relayring::capabilities::rr_0959_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0959: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0959: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0959: scanner should emit domain hints");
}
