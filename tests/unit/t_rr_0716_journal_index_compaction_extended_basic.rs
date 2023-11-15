//! Integration test for `RR-0716` (basic).
//! Extended: Journal index compaction extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0716_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd3, 0xd5];
    let first = relayring::capabilities::rr_0716_journal_index_compaction_extended::evaluate(fixture).expect("RR-0716: Extended: Journal index compaction extend codec v1");
    let second = relayring::capabilities::rr_0716_journal_index_compaction_extended::evaluate(fixture).expect("RR-0716: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0716: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0716: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
