//! Integration test for `RR-0237` (bounds).
//! Journal index compaction harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0237_journal_index_compaction_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0237_journal_index_compaction::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0237_journal_index_compaction::evaluate(fixture).expect("RR-0237 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
