//! Integration test for `RR-0220` (stability).
//! Journal index compaction validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0220_journal_index_compaction_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0xe1];
    let full = relayring::capabilities::rr_0220_journal_index_compaction::evaluate(fixture).expect("RR-0220: bulk Journal index compaction validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0220_journal_index_compaction::evaluate(&fixture[..end]).expect("RR-0220: stable prefix");
        assert!(partial.consumed <= end, "RR-0220: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0220: full prefix should match bulk checksum");
        }
    }
}
