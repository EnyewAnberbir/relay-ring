//! Integration test for `RR-0243` (basic).
//! Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0243_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0xf8];
    let first = relayring::capabilities::rr_0243_journal_index_compaction::evaluate(fixture).expect("RR-0243: Journal index compaction refactor mutator v28");
    let second = relayring::capabilities::rr_0243_journal_index_compaction::evaluate(fixture).expect("RR-0243: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0243: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0243: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
