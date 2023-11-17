//! Integration test for `RR-0744` (basic).
//! Extended: Journal index compaction benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0744_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef, 0xf1];
    let first = relayring::capabilities::rr_0744_journal_index_compaction_extended::evaluate(fixture).expect("RR-0744: Extended: Journal index compaction benchmark reporter v29");
    let second = relayring::capabilities::rr_0744_journal_index_compaction_extended::evaluate(fixture).expect("RR-0744: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0744: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0744: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
