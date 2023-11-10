//! Integration test for `RR-0687` (basic).
//! Extended: Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0687_journal_append_seal_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let first = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0687: Extended: Journal append seal harden index v12");
    let second = relayring::capabilities::rr_0687_journal_append_seal_hard_extended::evaluate(fixture).expect("RR-0687: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0687: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0687: scanner should emit domain hints");
}
