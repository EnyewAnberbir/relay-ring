//! Integration test for `RR-0692` (basic).
//! Extended: Journal append seal integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0692_journal_append_seal_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let first = relayring::capabilities::rr_0692_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0692: Extended: Journal append seal integrate validator v17");
    let second = relayring::capabilities::rr_0692_journal_append_seal_inte_extended::evaluate(fixture).expect("RR-0692: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0692: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0692: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
