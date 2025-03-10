//! Integration test for `RR-0222` (stability).
//! Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0222_journal_index_compaction_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let full = relayring::capabilities::rr_0222_journal_index_compaction::evaluate(fixture).expect("RR-0222: bulk Journal index compaction integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0222_journal_index_compaction::evaluate(&fixture[..end]).expect("RR-0222: stable prefix");
        assert!(partial.consumed <= end, "RR-0222: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0222: full prefix should match bulk checksum");
        }
    }
}
