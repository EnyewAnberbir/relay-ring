//! Integration test for `RR-0193` (basic).
//! Journal append seal refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0193_journal_append_seal_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let first = relayring::capabilities::rr_0193_journal_append_seal_refa::evaluate(fixture).expect("RR-0193: Journal append seal refactor mutator v18");
    let second = relayring::capabilities::rr_0193_journal_append_seal_refa::evaluate(fixture).expect("RR-0193: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0193: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0193: scanner should emit domain hints");
}
