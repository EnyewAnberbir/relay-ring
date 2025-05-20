//! Integration test for `RR-0728` (stability).
//! Extended: Journal index compaction wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0728_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0xe1];
    let full = relayring::capabilities::rr_0728_journal_index_compaction_extended::evaluate(fixture).expect("RR-0728: bulk Extended: Journal index compaction wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0728_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0728: stable prefix");
        assert!(partial.consumed <= end, "RR-0728: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0728: full prefix should match bulk checksum");
        }
    }
}
