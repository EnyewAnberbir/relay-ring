//! Integration test for `RR-0757` (empty).
//! Extended: Journal OTLP bridge integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0757_journal_otlp_bridge_inte_extended_empty() {
    assert!(relayring::capabilities::rr_0757_journal_otlp_bridge_inte_extended::evaluate(&[]).is_err(), "RR-0757: empty input must fail for Extended: Journal OTLP bridge integrate validator v7");
}
