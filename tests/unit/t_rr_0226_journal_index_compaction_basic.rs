//! Integration test for `RR-0226` (basic).
//! Journal index compaction extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0226_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe5, 0xe7];
    let first = relayring::capabilities::rr_0226_journal_index_compaction::evaluate(fixture).expect("RR-0226: Journal index compaction extend codec v11");
    let second = relayring::capabilities::rr_0226_journal_index_compaction::evaluate(fixture).expect("RR-0226: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0226: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0226: scanner should emit domain hints");
}
