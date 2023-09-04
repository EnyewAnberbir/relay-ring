//! Integration test for `RR-0183` (basic).
//! Journal append seal refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0183_journal_append_seal_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0xbc];
    let first = relayring::capabilities::rr_0183_journal_append_seal_refa::evaluate(fixture).expect("RR-0183: Journal append seal refactor mutator v8");
    let second = relayring::capabilities::rr_0183_journal_append_seal_refa::evaluate(fixture).expect("RR-0183: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0183: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0183: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
