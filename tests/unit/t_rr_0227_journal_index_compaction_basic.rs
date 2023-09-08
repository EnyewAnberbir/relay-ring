//! Integration test for `RR-0227` (basic).
//! Journal index compaction harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0227_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0xe8];
    let first = relayring::capabilities::rr_0227_journal_index_compaction::evaluate(fixture).expect("RR-0227: Journal index compaction harden index v12");
    let second = relayring::capabilities::rr_0227_journal_index_compaction::evaluate(fixture).expect("RR-0227: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0227: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0227: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
