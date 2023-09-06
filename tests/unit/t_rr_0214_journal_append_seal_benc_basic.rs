//! Integration test for `RR-0214` (basic).
//! Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0214_journal_append_seal_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let first = relayring::capabilities::rr_0214_journal_append_seal_benc::evaluate(fixture).expect("RR-0214: Journal append seal benchmark reporter v39");
    let second = relayring::capabilities::rr_0214_journal_append_seal_benc::evaluate(fixture).expect("RR-0214: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0214: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0214: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
