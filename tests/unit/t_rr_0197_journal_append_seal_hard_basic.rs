//! Integration test for `RR-0197` (basic).
//! Journal append seal harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0197_journal_append_seal_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let first = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(fixture).expect("RR-0197: Journal append seal harden index v22");
    let second = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(fixture).expect("RR-0197: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0197: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0197: scanner should emit domain hints");
}
