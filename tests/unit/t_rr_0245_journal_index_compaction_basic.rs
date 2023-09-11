//! Integration test for `RR-0245` (basic).
//! Journal index compaction implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0245_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0xfa];
    let first = relayring::capabilities::rr_0245_journal_index_compaction::evaluate(fixture).expect("RR-0245: Journal index compaction implement pipeline v30");
    let second = relayring::capabilities::rr_0245_journal_index_compaction::evaluate(fixture).expect("RR-0245: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0245: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0245: scanner should emit domain hints");
}
