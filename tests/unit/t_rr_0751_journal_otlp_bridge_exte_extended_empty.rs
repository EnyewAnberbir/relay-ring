//! Integration test for `RR-0751` (empty).
//! Extended: Journal OTLP bridge extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0751_journal_otlp_bridge_exte_extended_empty() {
    assert!(relayring::capabilities::rr_0751_journal_otlp_bridge_exte_extended::evaluate(&[]).is_err(), "RR-0751: empty input must fail for Extended: Journal OTLP bridge extend codec v1");
}
