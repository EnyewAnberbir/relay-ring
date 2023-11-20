//! Integration test for `RR-0759` (basic).
//! Extended: Journal OTLP bridge benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0759_journal_otlp_bridge_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let first = relayring::capabilities::rr_0759_journal_otlp_bridge_benc_extended::evaluate(fixture).expect("RR-0759: Extended: Journal OTLP bridge benchmark reporter v9");
    let second = relayring::capabilities::rr_0759_journal_otlp_bridge_benc_extended::evaluate(fixture).expect("RR-0759: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0759: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0759: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
