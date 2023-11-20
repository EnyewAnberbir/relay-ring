//! Integration test for `RR-0753` (basic).
//! Extended: Journal OTLP bridge wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0753_journal_otlp_bridge_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0xfa];
    let first = relayring::capabilities::rr_0753_journal_otlp_bridge_wire_extended::evaluate(fixture).expect("RR-0753: Extended: Journal OTLP bridge wire planner v3");
    let second = relayring::capabilities::rr_0753_journal_otlp_bridge_wire_extended::evaluate(fixture).expect("RR-0753: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0753: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0753: window consumes the whole buffer");
}
