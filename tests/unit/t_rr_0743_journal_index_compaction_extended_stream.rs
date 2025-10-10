//! Integration test for `RR-0743` (stream).
//! Extended: Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0743_journal_index_compaction_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xee, 0xf0];
    let direct = relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(fixture).expect("RR-0743: direct Extended: Journal index compaction refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(&copied).expect("RR-0743: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0743: stream path must consume input");
}
