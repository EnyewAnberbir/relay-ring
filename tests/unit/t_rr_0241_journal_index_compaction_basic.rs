//! Integration test for `RR-0241` (basic).
//! Journal index compaction export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0241_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4, 0xf6];
    let first = relayring::capabilities::rr_0241_journal_index_compaction::evaluate(fixture).expect("RR-0241: Journal index compaction export adapter v26");
    let second = relayring::capabilities::rr_0241_journal_index_compaction::evaluate(fixture).expect("RR-0241: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0241: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0241: scanner should emit domain hints");
}
