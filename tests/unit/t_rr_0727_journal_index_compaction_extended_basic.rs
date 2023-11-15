//! Integration test for `RR-0727` (basic).
//! Extended: Journal index compaction harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0727_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xde, 0xe0];
    let first = relayring::capabilities::rr_0727_journal_index_compaction_extended::evaluate(fixture).expect("RR-0727: Extended: Journal index compaction harden index v12");
    let second = relayring::capabilities::rr_0727_journal_index_compaction_extended::evaluate(fixture).expect("RR-0727: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0727: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0727: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
