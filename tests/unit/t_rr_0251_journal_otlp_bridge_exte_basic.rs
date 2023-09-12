//! Integration test for `RR-0251` (basic).
//! Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0251_journal_otlp_bridge_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x02];
    let first = relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0251: Journal OTLP bridge extend codec v1");
    let second = relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(fixture).expect("RR-0251: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0251: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0251: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
