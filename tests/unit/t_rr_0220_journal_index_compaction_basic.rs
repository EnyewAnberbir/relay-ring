//! Integration test for `RR-0220` (basic).
//! Journal index compaction validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0220_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0xe1];
    let first = relayring::capabilities::rr_0220_journal_index_compaction::evaluate(fixture).expect("RR-0220: Journal index compaction validate resolver v5");
    let second = relayring::capabilities::rr_0220_journal_index_compaction::evaluate(fixture).expect("RR-0220: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0220: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0220: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
