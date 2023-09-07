//! Integration test for `RR-0223` (basic).
//! Journal index compaction refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0223_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe2, 0xe4];
    let first = relayring::capabilities::rr_0223_journal_index_compaction::evaluate(fixture).expect("RR-0223: Journal index compaction refactor mutator v8");
    let second = relayring::capabilities::rr_0223_journal_index_compaction::evaluate(fixture).expect("RR-0223: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0223: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0223: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
