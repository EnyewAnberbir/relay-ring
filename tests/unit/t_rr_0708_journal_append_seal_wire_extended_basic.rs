//! Integration test for `RR-0708` (basic).
//! Extended: Journal append seal wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0708_journal_append_seal_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcb, 0xcd];
    let first = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0708: Extended: Journal append seal wire planner v33");
    let second = relayring::capabilities::rr_0708_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0708: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0708: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0708: window consumes the whole buffer");
}
