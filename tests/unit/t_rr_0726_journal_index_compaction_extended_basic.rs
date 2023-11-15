//! Integration test for `RR-0726` (basic).
//! Extended: Journal index compaction extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0726_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdd, 0xdf];
    let first = relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(fixture).expect("RR-0726: Extended: Journal index compaction extend codec v11");
    let second = relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(fixture).expect("RR-0726: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0726: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0726: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
