//! Integration test for `RR-0212` (basic).
//! Journal append seal integrate validator v37 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0212_journal_append_seal_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0xd9];
    let first = relayring::capabilities::rr_0212_journal_append_seal_inte::evaluate(fixture).expect("RR-0212: Journal append seal integrate validator v37");
    let second = relayring::capabilities::rr_0212_journal_append_seal_inte::evaluate(fixture).expect("RR-0212: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0212: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0212: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
