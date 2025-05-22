//! Integration test for `RR-0746` (stability).
//! Extended: Journal index compaction extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0746_journal_index_compaction_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let full = relayring::capabilities::rr_0746_journal_index_compaction_extended::evaluate(fixture).expect("RR-0746: bulk Extended: Journal index compaction extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0746_journal_index_compaction_extended::evaluate(&fixture[..end]).expect("RR-0746: stable prefix");
        assert!(partial.consumed <= end, "RR-0746: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0746: full prefix should match bulk checksum");
        }
    }
}
