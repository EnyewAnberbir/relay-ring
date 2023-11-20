//! Integration test for `RR-0761` (basic).
//! Extended: Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0761_journal_otlp_bridge_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04];
    let first = relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0761: Extended: Journal OTLP bridge extend codec v11");
    let second = relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(fixture).expect("RR-0761: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0761: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0761: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
