//! Integration test for `RR-0750` (basic).
//! Extended: Journal index compaction validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0750_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf5, 0xf7];
    let first = relayring::capabilities::rr_0750_journal_index_compaction_extended::evaluate(fixture).expect("RR-0750: Extended: Journal index compaction validate resolver v35");
    let second = relayring::capabilities::rr_0750_journal_index_compaction_extended::evaluate(fixture).expect("RR-0750: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0750: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0750: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
