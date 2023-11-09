//! Integration test for `RR-0683` (basic).
//! Extended: Journal append seal refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0683_journal_append_seal_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let first = relayring::capabilities::rr_0683_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0683: Extended: Journal append seal refactor mutator v8");
    let second = relayring::capabilities::rr_0683_journal_append_seal_refa_extended::evaluate(fixture).expect("RR-0683: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0683: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0683: scanner should emit domain hints");
}
