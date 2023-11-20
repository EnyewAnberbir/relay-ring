//! Integration test for `RR-0757` (basic).
//! Extended: Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0757_journal_otlp_bridge_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0xfe];
    let first = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(fixture).expect("RR-0757: Extended: Journal OTLP bridge integrate validator v7");
    let second = relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(fixture).expect("RR-0757: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0757: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0757: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
