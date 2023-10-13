//! Integration test for `RR-0474` (basic).
//! Runtime telemetry config fuzz CLI benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0474_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0xe1];
    let first = relayring::capabilities::rr_0474_runtime_telemetry_config::evaluate(fixture).expect("RR-0474: Runtime telemetry config fuzz CLI benchmark reporter v19");
    let second = relayring::capabilities::rr_0474_runtime_telemetry_config::evaluate(fixture).expect("RR-0474: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0474: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0474: scanner should emit domain hints");
}
