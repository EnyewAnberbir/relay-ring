//! Integration test for `RR-0680` (basic).
//! Extended: Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0680_journal_append_seal_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xb1];
    let first = relayring::capabilities::rr_0680_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0680: Extended: Journal append seal validate resolver v5");
    let second = relayring::capabilities::rr_0680_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0680: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0680: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0680: stats visits every byte");
}
