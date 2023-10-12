//! Integration test for `RR-0465` (basic).
//! Runtime telemetry config fuzz CLI implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0465_runtime_telemetry_config_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6, 0xd8];
    let first = relayring::capabilities::rr_0465_runtime_telemetry_config::evaluate(fixture).expect("RR-0465: Runtime telemetry config fuzz CLI implement pipeline v10");
    let second = relayring::capabilities::rr_0465_runtime_telemetry_config::evaluate(fixture).expect("RR-0465: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0465: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0465: window consumes the whole buffer");
}
