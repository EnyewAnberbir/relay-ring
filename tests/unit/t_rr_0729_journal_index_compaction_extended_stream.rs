//! Integration test for `RR-0729` (stream).
//! Extended: Journal index compaction optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0729_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let direct = relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(fixture).expect("RR-0729: direct Extended: Journal index compaction optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(&copied).expect("RR-0729: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0729: stream path must consume input");
}
