//! Integration test for `RR-0762` (basic).
//! Extended: Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0762_journal_otlp_bridge_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let first = relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("RR-0762: Extended: Journal OTLP bridge harden index v12");
    let second = relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(fixture).expect("RR-0762: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0762: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0762: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
