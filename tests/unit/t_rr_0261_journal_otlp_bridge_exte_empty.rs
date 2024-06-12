//! Integration test for `RR-0261` (empty).
//! Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0261_journal_otlp_bridge_exte_empty() {
    assert!(relayring::capabilities::rr_0261_journal_otlp_bridge_exte::evaluate(&[]).is_err(), "RR-0261: empty input must fail for Journal OTLP bridge extend codec v11");
}
