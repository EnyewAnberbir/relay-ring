//! Integration test for `RR-0234` (basic).
//! Journal index compaction benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0234_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xed, 0xef];
    let first = relayring::capabilities::rr_0234_journal_index_compaction::evaluate(fixture).expect("RR-0234: Journal index compaction benchmark reporter v19");
    let second = relayring::capabilities::rr_0234_journal_index_compaction::evaluate(fixture).expect("RR-0234: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0234: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0234: stats visits every byte");
}
