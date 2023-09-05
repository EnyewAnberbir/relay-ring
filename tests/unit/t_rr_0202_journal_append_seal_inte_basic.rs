//! Integration test for `RR-0202` (basic).
//! Journal append seal integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0202_journal_append_seal_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let first = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(fixture).expect("RR-0202: Journal append seal integrate validator v27");
    let second = relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(fixture).expect("RR-0202: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0202: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0202: scanner should emit domain hints");
}
