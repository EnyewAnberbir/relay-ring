//! Integration test for `RR-0205` (basic).
//! Journal append seal implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0205_journal_append_seal_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0xd2];
    let first = relayring::capabilities::rr_0205_journal_append_seal_impl::evaluate(fixture).expect("RR-0205: Journal append seal implement pipeline v30");
    let second = relayring::capabilities::rr_0205_journal_append_seal_impl::evaluate(fixture).expect("RR-0205: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0205: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0205: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
