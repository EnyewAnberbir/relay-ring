//! Integration test for `RR-0763` (basic).
//! Extended: Journal OTLP bridge wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0763_journal_otlp_bridge_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x06];
    let first = relayring::capabilities::rr_0763_journal_otlp_bridge_wire_extended::evaluate(fixture).expect("RR-0763: Extended: Journal OTLP bridge wire planner v13");
    let second = relayring::capabilities::rr_0763_journal_otlp_bridge_wire_extended::evaluate(fixture).expect("RR-0763: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0763: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0763: scanner should emit domain hints");
}
