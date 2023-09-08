//! Integration test for `RR-0232` (basic).
//! Journal index compaction integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0232_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xed];
    let first = relayring::capabilities::rr_0232_journal_index_compaction::evaluate(fixture).expect("RR-0232: Journal index compaction integrate validator v17");
    let second = relayring::capabilities::rr_0232_journal_index_compaction::evaluate(fixture).expect("RR-0232: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0232: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0232: scanner should emit domain hints");
}
