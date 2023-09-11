//! Integration test for `RR-0246` (basic).
//! Journal index compaction extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0246_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0xfb];
    let first = relayring::capabilities::rr_0246_journal_index_compaction::evaluate(fixture).expect("RR-0246: Journal index compaction extend codec v31");
    let second = relayring::capabilities::rr_0246_journal_index_compaction::evaluate(fixture).expect("RR-0246: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0246: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0246: scanner should emit domain hints");
}
