//! Integration test for `RR-0758` (empty).
//! Extended: Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0758_journal_otlp_bridge_refa_extended_empty() {
    assert!(relayring::capabilities::rr_0758_journal_otlp_bridge_refa_extended::evaluate(&[]).is_err(), "RR-0758: empty input must fail for Extended: Journal OTLP bridge refactor mutator v8");
}
