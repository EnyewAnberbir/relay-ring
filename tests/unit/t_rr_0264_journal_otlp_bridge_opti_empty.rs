//! Integration test for `RR-0264` (empty).
//! Journal OTLP bridge optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0264_journal_otlp_bridge_opti_empty() {
    assert!(relayring::capabilities::rr_0264_journal_otlp_bridge_opti::evaluate(&[]).is_err(), "RR-0264: empty input must fail for Journal OTLP bridge optimize registry v14");
}
