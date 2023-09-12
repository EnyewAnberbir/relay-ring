//! Integration test for `RR-0254` (basic).
//! Journal OTLP bridge optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0254_journal_otlp_bridge_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x05];
    let first = relayring::capabilities::rr_0254_journal_otlp_bridge_opti::evaluate(fixture).expect("RR-0254: Journal OTLP bridge optimize registry v4");
    let second = relayring::capabilities::rr_0254_journal_otlp_bridge_opti::evaluate(fixture).expect("RR-0254: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0254: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0254: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
