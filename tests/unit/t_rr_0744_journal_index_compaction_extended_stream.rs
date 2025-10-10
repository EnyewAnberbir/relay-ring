//! Integration test for `RR-0744` (stream).
//! Extended: Journal index compaction benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0744_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let direct = relayring::capabilities::rr_0744_journal_index_compaction_extended::evaluate(fixture).expect("RR-0744: direct Extended: Journal index compaction benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0744_journal_index_compaction_extended::evaluate(&copied).expect("RR-0744: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0744: stream path must consume input");
}
