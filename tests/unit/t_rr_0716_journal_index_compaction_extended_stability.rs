//! Integration test for `RR-0716` (stability).
//! Extended: Journal index compaction extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0716_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd3, 0xd5];
    let full = relayring::capabilities::rr_0716_journal_index_compaction_extended::evaluate(fixture).expect("RR-0716: bulk Extended: Journal index compaction extend codec v1");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0716_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0716: stable prefix");
        assert!(partial.consumed <= end, "RR-0716: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0716: full prefix should match bulk checksum");
        }
    }
}
