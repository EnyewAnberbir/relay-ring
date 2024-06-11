//! Integration test for `RR-0253` (empty).
//! Journal OTLP bridge wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0253_journal_otlp_bridge_wire_empty() {
    assert!(relayring::capabilities::rr_0253_journal_otlp_bridge_wire::evaluate(&[]).is_err(), "RR-0253: empty input must fail for Journal OTLP bridge wire planner v3");
}
