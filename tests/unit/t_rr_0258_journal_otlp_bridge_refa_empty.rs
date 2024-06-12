//! Integration test for `RR-0258` (empty).
//! Journal OTLP bridge refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0258_journal_otlp_bridge_refa_empty() {
    assert!(relayring::capabilities::rr_0258_journal_otlp_bridge_refa::evaluate(&[]).is_err(), "RR-0258: empty input must fail for Journal OTLP bridge refactor mutator v8");
}
