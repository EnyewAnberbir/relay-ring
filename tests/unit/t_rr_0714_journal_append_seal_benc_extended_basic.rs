//! Integration test for `RR-0714` (basic).
//! Extended: Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0714_journal_append_seal_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let first = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0714: Extended: Journal append seal benchmark reporter v39");
    let second = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0714: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0714: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0714: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
