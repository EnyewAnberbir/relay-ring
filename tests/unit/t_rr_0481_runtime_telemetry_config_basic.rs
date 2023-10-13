//! Integration test for `RR-0481` (basic).
//! Runtime telemetry config fuzz CLI export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0481_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0xe8];
    let first = relayring::capabilities::rr_0481_runtime_telemetry_config::evaluate(fixture).expect("RR-0481: Runtime telemetry config fuzz CLI export adapter v26");
    let second = relayring::capabilities::rr_0481_runtime_telemetry_config::evaluate(fixture).expect("RR-0481: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0481: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0481: scanner should emit domain hints");
}
