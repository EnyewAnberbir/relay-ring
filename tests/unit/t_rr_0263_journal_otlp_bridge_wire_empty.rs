//! Integration test for `RR-0263` (empty).
//! Journal OTLP bridge wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0263_journal_otlp_bridge_wire_empty() {
    assert!(relayring::capabilities::rr_0263_journal_otlp_bridge_wire::evaluate(&[]).is_err(), "RR-0263: empty input must fail for Journal OTLP bridge wire planner v13");
}
