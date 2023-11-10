//! Integration test for `RR-0688` (basic).
//! Extended: Journal append seal wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0688_journal_append_seal_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let first = relayring::capabilities::rr_0688_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0688: Extended: Journal append seal wire planner v13");
    let second = relayring::capabilities::rr_0688_journal_append_seal_wire_extended::evaluate(fixture).expect("RR-0688: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0688: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0688: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
