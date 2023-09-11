//! Integration test for `RR-0244` (basic).
//! Journal index compaction benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0244_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf7, 0xf9];
    let first = relayring::capabilities::rr_0244_journal_index_compaction::evaluate(fixture).expect("RR-0244: Journal index compaction benchmark reporter v29");
    let second = relayring::capabilities::rr_0244_journal_index_compaction::evaluate(fixture).expect("RR-0244: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0244: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0244: scanner should emit domain hints");
}
