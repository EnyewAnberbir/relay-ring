//! Integration test for `RR-0976` (basic).
//! Extended: Runtime telemetry config fuzz CLI extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0976_runtime_telemetry_config_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let first = relayring::capabilities::rr_0976_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0976: Extended: Runtime telemetry config fuzz CLI extend codec v21");
    let second = relayring::capabilities::rr_0976_runtime_telemetry_config_extended::evaluate(fixture).expect("RR-0976: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0976: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0976: scanner should emit domain hints");
}
