//! Integration test for `RR-0735` (basic).
//! Extended: Journal index compaction implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0735_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe6, 0xe8];
    let first = relayring::capabilities::rr_0735_journal_index_compaction_extended::evaluate(fixture).expect("RR-0735: Extended: Journal index compaction implement pipeline v20");
    let second = relayring::capabilities::rr_0735_journal_index_compaction_extended::evaluate(fixture).expect("RR-0735: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0735: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0735: scanner should emit domain hints");
}
