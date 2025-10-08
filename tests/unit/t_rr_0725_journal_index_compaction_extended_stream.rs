//! Integration test for `RR-0725` (stream).
//! Extended: Journal index compaction implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0725_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdc, 0xde];
    let direct = relayring::capabilities::rr_0725_journal_index_compaction_extended::evaluate(fixture).expect("RR-0725: direct Extended: Journal index compaction implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0725_journal_index_compaction_extended::evaluate(&copied).expect("RR-0725: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0725: stream path must consume input");
}
