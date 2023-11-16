//! Integration test for `RR-0740` (basic).
//! Extended: Journal index compaction validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0740_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xeb, 0xed];
    let first = relayring::capabilities::rr_0740_journal_index_compaction_extended::evaluate(fixture).expect("RR-0740: Extended: Journal index compaction validate resolver v25");
    let second = relayring::capabilities::rr_0740_journal_index_compaction_extended::evaluate(fixture).expect("RR-0740: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0740: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0740: scanner should emit domain hints");
}
