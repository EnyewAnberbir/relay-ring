//! Integration test for `RR-0225` (basic).
//! Journal index compaction implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0225_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0xe6];
    let first = relayring::capabilities::rr_0225_journal_index_compaction::evaluate(fixture).expect("RR-0225: Journal index compaction implement pipeline v10");
    let second = relayring::capabilities::rr_0225_journal_index_compaction::evaluate(fixture).expect("RR-0225: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0225: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0225: scanner should emit domain hints");
}
