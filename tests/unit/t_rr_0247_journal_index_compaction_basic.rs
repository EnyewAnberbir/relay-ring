//! Integration test for `RR-0247` (basic).
//! Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0247_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let first = relayring::capabilities::rr_0247_journal_index_compaction::evaluate(fixture).expect("RR-0247: Journal index compaction harden index v32");
    let second = relayring::capabilities::rr_0247_journal_index_compaction::evaluate(fixture).expect("RR-0247: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0247: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0247: scanner should emit domain hints");
}
