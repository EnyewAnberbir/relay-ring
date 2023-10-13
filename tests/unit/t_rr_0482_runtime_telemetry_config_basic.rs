//! Integration test for `RR-0482` (basic).
//! Runtime telemetry config fuzz CLI integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0482_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0xe9];
    let first = relayring::capabilities::rr_0482_runtime_telemetry_config::evaluate(fixture).expect("RR-0482: Runtime telemetry config fuzz CLI integrate validator v27");
    let second = relayring::capabilities::rr_0482_runtime_telemetry_config::evaluate(fixture).expect("RR-0482: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0482: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0482: scanner should emit domain hints");
}
