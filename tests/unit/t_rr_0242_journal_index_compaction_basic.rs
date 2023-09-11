//! Integration test for `RR-0242` (basic).
//! Journal index compaction integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0242_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf5, 0xf7];
    let first = relayring::capabilities::rr_0242_journal_index_compaction::evaluate(fixture).expect("RR-0242: Journal index compaction integrate validator v27");
    let second = relayring::capabilities::rr_0242_journal_index_compaction::evaluate(fixture).expect("RR-0242: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0242: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0242: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
