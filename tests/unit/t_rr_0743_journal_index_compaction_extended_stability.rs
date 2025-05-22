//! Integration test for `RR-0743` (stability).
//! Extended: Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0743_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xee, 0xf0];
    let full = relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(fixture).expect("RR-0743: bulk Extended: Journal index compaction refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0743: stable prefix");
        assert!(partial.consumed <= end, "RR-0743: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0743: full prefix should match bulk checksum");
        }
    }
}
