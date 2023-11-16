//! Integration test for `RR-0731` (basic).
//! Extended: Journal index compaction export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0731_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe2, 0xe4];
    let first = relayring::capabilities::rr_0731_journal_index_compaction_extended::evaluate(fixture).expect("RR-0731: Extended: Journal index compaction export adapter v16");
    let second = relayring::capabilities::rr_0731_journal_index_compaction_extended::evaluate(fixture).expect("RR-0731: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0731: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0731: stats visits every byte");
}
