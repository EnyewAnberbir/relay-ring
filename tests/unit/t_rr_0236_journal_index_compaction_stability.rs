//! Integration test for `RR-0236` (stability).
//! Journal index compaction extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0236_journal_index_compaction_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let full = relayring::capabilities::rr_0236_journal_index_compaction::evaluate(fixture).expect("RR-0236: bulk Journal index compaction extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0236_journal_index_compaction::evaluate(&fixture[..end]).expect("RR-0236: stable prefix");
        assert!(partial.consumed <= end, "RR-0236: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0236: full prefix should match bulk checksum");
        }
    }
}
