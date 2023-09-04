//! Integration test for `RR-0192` (basic).
//! Journal append seal integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0192_journal_append_seal_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc3, 0xc5];
    let first = relayring::capabilities::rr_0192_journal_append_seal_inte::evaluate(fixture).expect("RR-0192: Journal append seal integrate validator v17");
    let second = relayring::capabilities::rr_0192_journal_append_seal_inte::evaluate(fixture).expect("RR-0192: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0192: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0192: stats visits every byte");
}
