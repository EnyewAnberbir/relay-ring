//! Integration test for `RR-0258` (basic).
//! Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0258_journal_otlp_bridge_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x09];
    let first = relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(fixture).expect("RR-0258: Journal OTLP bridge refactor mutator v8");
    let second = relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(fixture).expect("RR-0258: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0258: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0258: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
