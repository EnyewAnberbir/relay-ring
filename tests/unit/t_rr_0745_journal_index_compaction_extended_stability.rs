//! Integration test for `RR-0745` (stability).
//! Extended: Journal index compaction implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0745_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xf2];
    let full = relayring::capabilities::rr_0745_journal_index_compaction_extended::evaluate(fixture).expect("RR-0745: bulk Extended: Journal index compaction implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0745_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0745: stable prefix");
        assert!(partial.consumed <= end, "RR-0745: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0745: full prefix should match bulk checksum");
        }
    }
}
