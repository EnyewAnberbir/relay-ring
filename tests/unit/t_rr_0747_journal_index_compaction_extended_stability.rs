//! Integration test for `RR-0747` (stability).
//! Extended: Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0747_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf2, 0xf4];
    let full = relayring::capabilities::rr_0747_journal_index_compaction_extended::evaluate(fixture).expect("RR-0747: bulk Extended: Journal index compaction harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0747_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0747: stable prefix");
        assert!(partial.consumed <= end, "RR-0747: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0747: full prefix should match bulk checksum");
        }
    }
}
