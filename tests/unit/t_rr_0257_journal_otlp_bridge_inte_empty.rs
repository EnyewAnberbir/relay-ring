//! Integration test for `RR-0257` (empty).
//! Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0257_journal_otlp_bridge_inte_empty() {
    assert!(relayring::capabilities::rr_0257_journal_otlp_bridge_inte::evaluate(&[]).is_err(), "RR-0257: empty input must fail for Journal OTLP bridge integrate validator v7");
}
