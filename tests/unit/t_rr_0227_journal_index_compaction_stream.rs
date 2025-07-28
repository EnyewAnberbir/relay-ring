//! Integration test for `RR-0227` (stream).
//! Journal index compaction harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0227_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0xe8];
    let direct = relayring::capabilities::rr_0227_journal_index_compaction::evaluate(fixture).expect("RR-0227: direct Journal index compaction harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0227_journal_index_compaction::evaluate(&copied).expect("RR-0227: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0227: stream path must consume input");
}
