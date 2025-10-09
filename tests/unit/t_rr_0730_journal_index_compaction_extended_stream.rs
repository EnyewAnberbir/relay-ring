//! Integration test for `RR-0730` (stream).
//! Extended: Journal index compaction validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0730_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let direct = relayring::capabilities::rr_0730_journal_index_compaction_extended::evaluate(fixture).expect("RR-0730: direct Extended: Journal index compaction validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0730_journal_index_compaction_extended::evaluate(&copied).expect("RR-0730: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0730: stream path must consume input");
}
