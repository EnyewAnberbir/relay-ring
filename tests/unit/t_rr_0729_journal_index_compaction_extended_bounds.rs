//! Integration test for `RR-0729` (bounds).
//! Extended: Journal index compaction optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0729_journal_index_compaction_extended_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(fixture).expect("RR-0729 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
