//! Integration test for `RR-0747` (basic).
//! Extended: Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0747_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf2, 0xf4];
    let first = relayring::capabilities::rr_0747_journal_index_compaction_extended::evaluate(fixture).expect("RR-0747: Extended: Journal index compaction harden index v32");
    let second = relayring::capabilities::rr_0747_journal_index_compaction_extended::evaluate(fixture).expect("RR-0747: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0747: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0747: scanner should emit domain hints");
}
