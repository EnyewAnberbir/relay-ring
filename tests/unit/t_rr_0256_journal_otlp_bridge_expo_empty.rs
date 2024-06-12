//! Integration test for `RR-0256` (empty).
//! Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0256_journal_otlp_bridge_expo_empty() {
    assert!(relayring::capabilities::rr_0256_journal_otlp_bridge_expo::evaluate(&[]).is_err(), "RR-0256: empty input must fail for Journal OTLP bridge export adapter v6");
}
