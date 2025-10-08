//! Integration test for `RR-0726` (stream).
//! Extended: Journal index compaction extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0726_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdd, 0xdf];
    let direct = relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(fixture).expect("RR-0726: direct Extended: Journal index compaction extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(&copied).expect("RR-0726: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0726: stream path must consume input");
}
