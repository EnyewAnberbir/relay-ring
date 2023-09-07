//! Integration test for `RR-0221` (basic).
//! Journal index compaction export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0221_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let first = relayring::capabilities::rr_0221_journal_index_compaction::evaluate(fixture).expect("RR-0221: Journal index compaction export adapter v6");
    let second = relayring::capabilities::rr_0221_journal_index_compaction::evaluate(fixture).expect("RR-0221: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0221: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0221: stats visits every byte");
}
