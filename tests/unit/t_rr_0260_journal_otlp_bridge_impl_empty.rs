//! Integration test for `RR-0260` (empty).
//! Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0260_journal_otlp_bridge_impl_empty() {
    assert!(relayring::capabilities::rr_0260_journal_otlp_bridge_impl::evaluate(&[]).is_err(), "RR-0260: empty input must fail for Journal OTLP bridge implement pipeline v10");
}
