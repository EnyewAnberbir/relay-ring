//! Integration test for `RR-0723` (stability).
//! Extended: Journal index compaction refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0723_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let full = relayring::capabilities::rr_0723_journal_index_compaction_extended::evaluate(fixture).expect("RR-0723: bulk Extended: Journal index compaction refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0723_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0723: stable prefix");
        assert!(partial.consumed <= end, "RR-0723: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0723: full prefix should match bulk checksum");
        }
    }
}
