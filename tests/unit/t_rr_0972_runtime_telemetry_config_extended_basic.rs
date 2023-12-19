//! Integration test for `RR-0972` (basic).
//! Extended: Runtime telemetry config fuzz CLI integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0972_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd5, 0xd7];
    let first = relayring::capabilities::rr_0972_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0972: Extended: Runtime telemetry config fuzz CLI integrate validator v17");
    let second = relayring::capabilities::rr_0972_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0972: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0972: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0972: scanner should emit domain hints");
}
