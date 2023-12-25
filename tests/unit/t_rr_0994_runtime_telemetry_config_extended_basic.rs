//! Integration test for `RR-0994` (basic).
//! Extended: Runtime telemetry config fuzz CLI benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0994_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xed];
    let first = relayring::capabilities::rr_0994_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0994: Extended: Runtime telemetry config fuzz CLI benchmark reporter v39");
    let second = relayring::capabilities::rr_0994_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0994: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0994: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0994: scanner should emit domain hints");
}
