//! Integration test for `RR-0706` (basic).
//! Extended: Journal append seal extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0706_journal_append_seal_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc9, 0xcb];
    let first = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0706: Extended: Journal append seal extend codec v31");
    let second = relayring::capabilities::rr_0706_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0706: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0706: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0706: stats visits every byte");
}
