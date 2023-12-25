//! Integration test for `RR-0989` (basic).
//! Extended: Runtime telemetry config fuzz CLI optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0989_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0xe8];
    let first = relayring::capabilities::rr_0989_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0989: Extended: Runtime telemetry config fuzz CLI optimize registry v34");
    let second = relayring::capabilities::rr_0989_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0989: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0989: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0989: scanner should emit domain hints");
}
