//! Integration test for `RR-0235` (basic).
//! Journal index compaction implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0235_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xee, 0xf0];
    let first = relayring::capabilities::rr_0235_journal_index_compaction::evaluate(fixture).expect("RR-0235: Journal index compaction implement pipeline v20");
    let second = relayring::capabilities::rr_0235_journal_index_compaction::evaluate(fixture).expect("RR-0235: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0235: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0235: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
