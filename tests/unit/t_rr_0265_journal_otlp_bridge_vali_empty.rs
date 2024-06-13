//! Integration test for `RR-0265` (empty).
//! Journal OTLP bridge validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0265_journal_otlp_bridge_vali_empty() {
    assert!(relayring::capabilities::rr_0265_journal_otlp_bridge_vali::evaluate(&[]).is_err(), "RR-0265: empty input must fail for Journal OTLP bridge validate resolver v15");
}
