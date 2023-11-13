//! Integration test for `RR-0697` (basic).
//! Extended: Journal append seal harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0697_journal_append_seal_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xc2];
    let first = relayring::capabilities::rr_0697_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0697: Extended: Journal append seal harden index v22");
    let second = relayring::capabilities::rr_0697_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0697: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0697: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0697: scanner should emit domain hints");
}
