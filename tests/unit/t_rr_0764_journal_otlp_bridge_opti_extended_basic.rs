//! Integration test for `RR-0764` (basic).
//! Extended: Journal OTLP bridge optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0764_journal_otlp_bridge_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x07];
    let first = relayring::capabilities::rr_0764_journal_otlp_bridge_opti_extended::evaluate(fixture).expect("RR-0764: Extended: Journal OTLP bridge optimize registry v14");
    let second = relayring::capabilities::rr_0764_journal_otlp_bridge_opti_extended::evaluate(fixture).expect("RR-0764: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0764: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0764: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
