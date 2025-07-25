//! Integration test for `RR-0217` (stream).
//! Journal index compaction harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0217_journal_index_compaction_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdc, 0xde];
    let direct = relayring::capabilities::rr_0217_journal_index_compaction::evaluate(fixture).expect("RR-0217: direct Journal index compaction harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0217_journal_index_compaction::evaluate(&copied).expect("RR-0217: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0217: stream path must consume input");
}
