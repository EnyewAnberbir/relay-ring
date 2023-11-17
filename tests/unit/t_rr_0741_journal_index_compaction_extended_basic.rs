//! Integration test for `RR-0741` (basic).
//! Extended: Journal index compaction export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0741_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xec, 0xee];
    let first = relayring::capabilities::rr_0741_journal_index_compaction_extended::evaluate(fixture).expect("RR-0741: Extended: Journal index compaction export adapter v26");
    let second = relayring::capabilities::rr_0741_journal_index_compaction_extended::evaluate(fixture).expect("RR-0741: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0741: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0741: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
