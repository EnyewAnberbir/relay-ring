//! Integration test for `RR-0247` (stream).
//! Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0247_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0xfc];
    let direct = relayring::capabilities::rr_0247_journal_index_compaction::evaluate(fixture).expect("RR-0247: direct Journal index compaction harden index v32");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0247_journal_index_compaction::evaluate(&copied).expect("RR-0247: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0247: stream path must consume input");
}
