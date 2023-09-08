//! Integration test for `RR-0236` (basic).
//! Journal index compaction extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0236_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let first = relayring::capabilities::rr_0236_journal_index_compaction::evaluate(fixture).expect("RR-0236: Journal index compaction extend codec v21");
    let second = relayring::capabilities::rr_0236_journal_index_compaction::evaluate(fixture).expect("RR-0236: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0236: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0236: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
