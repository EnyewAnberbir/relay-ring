//! Integration test for `RR-0223` (stream).
//! Journal index compaction refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0223_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe2, 0xe4];
    let direct = relayring::capabilities::rr_0223_journal_index_compaction::evaluate(fixture).expect("RR-0223: direct Journal index compaction refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0223_journal_index_compaction::evaluate(&copied).expect("RR-0223: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0223: stream path must consume input");
}
