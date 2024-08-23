//! Integration test for `RR-0760` (empty).
//! Extended: Journal OTLP bridge implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0760_journal_otlp_bridge_impl_extended_empty() {
    assert!(relayring::capabilities::rr_0760_journal_otlp_bridge_impl_extended::evaluate(&[]).is_err(), "RR-0760: empty input must fail for Extended: Journal OTLP bridge implement pipeline v10");
}
