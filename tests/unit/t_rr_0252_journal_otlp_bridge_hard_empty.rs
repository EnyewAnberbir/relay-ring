//! Integration test for `RR-0252` (empty).
//! Journal OTLP bridge harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0252_journal_otlp_bridge_hard_empty() {
    assert!(relayring::capabilities::rr_0252_journal_otlp_bridge_hard::evaluate(&[]).is_err(), "RR-0252: empty input must fail for Journal OTLP bridge harden index v2");
}
