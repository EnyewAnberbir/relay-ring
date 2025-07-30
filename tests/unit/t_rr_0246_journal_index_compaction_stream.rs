//! Integration test for `RR-0246` (stream).
//! Journal index compaction extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0246_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0xfb];
    let direct = relayring::capabilities::rr_0246_journal_index_compaction::evaluate(fixture).expect("RR-0246: direct Journal index compaction extend codec v31");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0246_journal_index_compaction::evaluate(&copied).expect("RR-0246: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0246: stream path must consume input");
}
