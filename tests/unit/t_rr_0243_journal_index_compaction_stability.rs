//! Integration test for `RR-0243` (stability).
//! Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0243_journal_index_compaction_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0xf8];
    let full = relayring::capabilities::rr_0243_journal_index_compaction::evaluate(fixture).expect("RR-0243: bulk Journal index compaction refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0243_journal_index_compaction::evaluate(&fixture[..end]).expect("RR-0243: stable prefix");
        assert!(partial.consumed <= end, "RR-0243: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0243: full prefix should match bulk checksum");
        }
    }
}
