//! Integration test for `RR-0730` (basic).
//! Extended: Journal index compaction validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0730_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let first = relayring::capabilities::rr_0730_journal_index_compaction_extended::evaluate(fixture).expect("RR-0730: Extended: Journal index compaction validate resolver v15");
    let second = relayring::capabilities::rr_0730_journal_index_compaction_extended::evaluate(fixture).expect("RR-0730: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0730: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0730: scanner should emit domain hints");
}
