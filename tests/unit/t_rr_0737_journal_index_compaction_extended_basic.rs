//! Integration test for `RR-0737` (basic).
//! Extended: Journal index compaction harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0737_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8, 0xea];
    let first = relayring::capabilities::rr_0737_journal_index_compaction_extended::evaluate(fixture).expect("RR-0737: Extended: Journal index compaction harden index v22");
    let second = relayring::capabilities::rr_0737_journal_index_compaction_extended::evaluate(fixture).expect("RR-0737: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0737: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0737: stats visits every byte");
}
