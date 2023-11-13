//! Integration test for `RR-0698` (basic).
//! Extended: Journal append seal wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0698_journal_append_seal_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let first = relayring::capabilities::rr_0698_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0698: Extended: Journal append seal wire planner v23");
    let second = relayring::capabilities::rr_0698_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0698: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0698: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0698: stats visits every byte");
}
