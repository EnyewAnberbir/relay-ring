//! Integration test for `RR-0247` (stability).
//! Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0247_journal_index_compaction_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let full = relayring::capabilities::rr_0247_journal_index_compaction::evaluate(fixture).expect("RR-0247: bulk Journal index compaction harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0247_journal_index_compaction::evaluate(&fixture[..end]).expect("RR-0247: stable prefix");
        assert!(partial.consumed <= end, "RR-0247: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0247: full prefix should match bulk checksum");
        }
    }
}
