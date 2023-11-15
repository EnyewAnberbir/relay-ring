//! Integration test for `RR-0722` (basic).
//! Extended: Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0722_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let first = relayring::capabilities::rr_0722_journal_index_compaction_extended::evaluate(fixture).expect("RR-0722: Extended: Journal index compaction integrate validator v7");
    let second = relayring::capabilities::rr_0722_journal_index_compaction_extended::evaluate(fixture).expect("RR-0722: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0722: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0722: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
