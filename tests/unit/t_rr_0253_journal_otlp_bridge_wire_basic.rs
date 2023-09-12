//! Integration test for `RR-0253` (basic).
//! Journal OTLP bridge wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0253_journal_otlp_bridge_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x04];
    let first = relayring::capabilities::rr_0253_journal_otlp_bridge_wire::evaluate(fixture).expect("RR-0253: Journal OTLP bridge wire planner v3");
    let second = relayring::capabilities::rr_0253_journal_otlp_bridge_wire::evaluate(fixture).expect("RR-0253: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0253: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0253: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
