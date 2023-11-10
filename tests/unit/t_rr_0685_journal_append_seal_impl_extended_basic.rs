//! Integration test for `RR-0685` (basic).
//! Extended: Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0685_journal_append_seal_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let first = relayring::capabilities::rr_0685_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0685: Extended: Journal append seal implement pipeline v10");
    let second = relayring::capabilities::rr_0685_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0685: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0685: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0685: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
