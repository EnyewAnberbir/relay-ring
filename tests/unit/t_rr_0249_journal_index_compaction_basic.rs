//! Integration test for `RR-0249` (basic).
//! Journal index compaction optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0249_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let first = relayring::capabilities::rr_0249_journal_index_compaction::evaluate(fixture).expect("RR-0249: Journal index compaction optimize registry v34");
    let second = relayring::capabilities::rr_0249_journal_index_compaction::evaluate(fixture).expect("RR-0249: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0249: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0249: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
