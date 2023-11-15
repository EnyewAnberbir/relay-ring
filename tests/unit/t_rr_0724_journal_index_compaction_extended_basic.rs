//! Integration test for `RR-0724` (basic).
//! Extended: Journal index compaction benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0724_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdb, 0xdd];
    let first = relayring::capabilities::rr_0724_journal_index_compaction_extended::evaluate(fixture).expect("RR-0724: Extended: Journal index compaction benchmark reporter v9");
    let second = relayring::capabilities::rr_0724_journal_index_compaction_extended::evaluate(fixture).expect("RR-0724: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0724: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0724: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
