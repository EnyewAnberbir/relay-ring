//! Integration test for `RR-0255` (empty).
//! Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0255_journal_otlp_bridge_vali_empty() {
    assert!(relayring::capabilities::rr_0255_journal_otlp_bridge_vali::evaluate(&[]).is_err(), "RR-0255: empty input must fail for Journal OTLP bridge validate resolver v5");
}
