//! Integration test for `RR-0741` (stream).
//! Extended: Journal index compaction export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0741_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xec, 0xee];
    let direct = relayring::capabilities::rr_0741_journal_index_compaction_extended::evaluate(fixture).expect("RR-0741: direct Extended: Journal index compaction export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0741_journal_index_compaction_extended::evaluate(&copied).expect("RR-0741: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0741: stream path must consume input");
}
