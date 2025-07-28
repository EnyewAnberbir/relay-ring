//! Integration test for `RR-0222` (stream).
//! Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0222_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let direct = relayring::capabilities::rr_0222_journal_index_compaction::evaluate(fixture).expect("RR-0222: direct Journal index compaction integrate validator v7");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0222_journal_index_compaction::evaluate(&copied).expect("RR-0222: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0222: stream path must consume input");
}
