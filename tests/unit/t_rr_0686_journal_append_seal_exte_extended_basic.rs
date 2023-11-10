//! Integration test for `RR-0686` (basic).
//! Extended: Journal append seal extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0686_journal_append_seal_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let first = relayring::capabilities::rr_0686_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0686: Extended: Journal append seal extend codec v11");
    let second = relayring::capabilities::rr_0686_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0686: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0686: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0686: stats visits every byte");
}
