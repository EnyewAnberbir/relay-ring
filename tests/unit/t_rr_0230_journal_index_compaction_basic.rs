//! Integration test for `RR-0230` (basic).
//! Journal index compaction validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0230_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let first = relayring::capabilities::rr_0230_journal_index_compaction::evaluate(fixture).expect("RR-0230: Journal index compaction validate resolver v15");
    let second = relayring::capabilities::rr_0230_journal_index_compaction::evaluate(fixture).expect("RR-0230: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0230: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0230: stats visits every byte");
}
