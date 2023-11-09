//! Integration test for `RR-0684` (basic).
//! Extended: Journal append seal benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0684_journal_append_seal_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let first = relayring::capabilities::rr_0684_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0684: Extended: Journal append seal benchmark reporter v9");
    let second = relayring::capabilities::rr_0684_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0684: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0684: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0684: stats visits every byte");
}
