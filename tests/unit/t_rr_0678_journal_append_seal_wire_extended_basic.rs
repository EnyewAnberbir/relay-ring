//! Integration test for `RR-0678` (basic).
//! Extended: Journal append seal wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0678_journal_append_seal_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xad, 0xaf];
    let first = relayring::capabilities::rr_0678_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0678: Extended: Journal append seal wire planner v3");
    let second = relayring::capabilities::rr_0678_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0678: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0678: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0678: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
