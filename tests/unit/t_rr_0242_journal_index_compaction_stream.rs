//! Integration test for `RR-0242` (stream).
//! Journal index compaction integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0242_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf5, 0xf7];
    let direct = relayring::capabilities::rr_0242_journal_index_compaction::evaluate(fixture).expect("RR-0242: direct Journal index compaction integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0242_journal_index_compaction::evaluate(&copied).expect("RR-0242: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0242: stream path must consume input");
}
