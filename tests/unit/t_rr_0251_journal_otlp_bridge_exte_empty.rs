//! Integration test for `RR-0251` (empty).
//! Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0251_journal_otlp_bridge_exte_empty() {
    assert!(relayring::capabilities::rr_0251_journal_otlp_bridge_exte::evaluate(&[]).is_err(), "RR-0251: empty input must fail for Journal OTLP bridge extend codec v1");
}
