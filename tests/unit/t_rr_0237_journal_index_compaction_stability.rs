//! Integration test for `RR-0237` (stability).
//! Journal index compaction harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0237_journal_index_compaction_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xf2];
    let full = relayring::capabilities::rr_0237_journal_index_compaction::evaluate(fixture).expect("RR-0237: bulk Journal index compaction harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0237_journal_index_compaction::evaluate(&fixture[..end]).expect("RR-0237: stable prefix");
        assert!(partial.consumed <= end, "RR-0237: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0237: full prefix should match bulk checksum");
        }
    }
}
