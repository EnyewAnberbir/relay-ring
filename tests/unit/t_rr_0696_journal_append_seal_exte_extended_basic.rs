//! Integration test for `RR-0696` (basic).
//! Extended: Journal append seal extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0696_journal_append_seal_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0xc1];
    let first = relayring::capabilities::rr_0696_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0696: Extended: Journal append seal extend codec v21");
    let second = relayring::capabilities::rr_0696_journal_append_seal_exte_extended::evaluate(fixture).expect("RR-0696: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0696: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0696: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
