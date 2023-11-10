//! Integration test for `RR-0690` (basic).
//! Extended: Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0690_journal_append_seal_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb9, 0xbb];
    let first = relayring::capabilities::rr_0690_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0690: Extended: Journal append seal validate resolver v15");
    let second = relayring::capabilities::rr_0690_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0690: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0690: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0690: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
