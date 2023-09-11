//! Integration test for `RR-0250` (basic).
//! Journal index compaction validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0250_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x01];
    let first = relayring::capabilities::rr_0250_journal_index_compaction::evaluate(fixture).expect("RR-0250: Journal index compaction validate resolver v35");
    let second = relayring::capabilities::rr_0250_journal_index_compaction::evaluate(fixture).expect("RR-0250: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0250: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0250: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
