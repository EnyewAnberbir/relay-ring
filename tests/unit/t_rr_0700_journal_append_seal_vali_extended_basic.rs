//! Integration test for `RR-0700` (basic).
//! Extended: Journal append seal validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0700_journal_append_seal_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0xc5];
    let first = relayring::capabilities::rr_0700_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0700: Extended: Journal append seal validate resolver v25");
    let second = relayring::capabilities::rr_0700_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0700: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0700: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0700: scanner should emit domain hints");
}
