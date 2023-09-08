//! Integration test for `RR-0237` (basic).
//! Journal index compaction harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0237_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xf2];
    let first = relayring::capabilities::rr_0237_journal_index_compaction::evaluate(fixture).expect("RR-0237: Journal index compaction harden index v22");
    let second = relayring::capabilities::rr_0237_journal_index_compaction::evaluate(fixture).expect("RR-0237: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0237: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0237: stats visits every byte");
}
