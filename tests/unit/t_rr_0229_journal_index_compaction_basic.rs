//! Integration test for `RR-0229` (basic).
//! Journal index compaction optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0229_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8, 0xea];
    let first = relayring::capabilities::rr_0229_journal_index_compaction::evaluate(fixture).expect("RR-0229: Journal index compaction optimize registry v14");
    let second = relayring::capabilities::rr_0229_journal_index_compaction::evaluate(fixture).expect("RR-0229: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0229: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0229: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
