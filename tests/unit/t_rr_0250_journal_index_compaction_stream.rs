//! Integration test for `RR-0250` (stream).
//! Journal index compaction validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0250_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x01];
    let direct = relayring::capabilities::rr_0250_journal_index_compaction::evaluate(fixture).expect("RR-0250: direct Journal index compaction validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0250_journal_index_compaction::evaluate(&copied).expect("RR-0250: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0250: stream path must consume input");
}
