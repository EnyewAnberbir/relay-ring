//! Integration test for `RR-0763` (empty).
//! Extended: Journal OTLP bridge wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0763_journal_otlp_bridge_wire_extended_empty() {
    assert!(relayring::capabilities::rr_0763_journal_otlp_bridge_wire_extended::evaluate(&[]).is_err(), "RR-0763: empty input must fail for Extended: Journal OTLP bridge wire planner v13");
}
