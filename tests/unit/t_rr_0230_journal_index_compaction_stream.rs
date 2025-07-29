//! Integration test for `RR-0230` (stream).
//! Journal index compaction validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0230_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let direct = relayring::capabilities::rr_0230_journal_index_compaction::evaluate(fixture).expect("RR-0230: direct Journal index compaction validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0230_journal_index_compaction::evaluate(&copied).expect("RR-0230: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0230: stream path must consume input");
}
