//! Integration test for `RR-0748` (stream).
//! Extended: Journal index compaction wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0748_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf3, 0xf5];
    let direct = relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(fixture).expect("RR-0748: direct Extended: Journal index compaction wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(&copied).expect("RR-0748: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0748: stream path must consume input");
}
