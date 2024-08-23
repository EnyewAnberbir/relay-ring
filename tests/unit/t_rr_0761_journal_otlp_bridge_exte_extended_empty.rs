//! Integration test for `RR-0761` (empty).
//! Extended: Journal OTLP bridge extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0761_journal_otlp_bridge_exte_extended_empty() {
    assert!(relayring::capabilities::rr_0761_journal_otlp_bridge_exte_extended::evaluate(&[]).is_err(), "RR-0761: empty input must fail for Extended: Journal OTLP bridge extend codec v11");
}
