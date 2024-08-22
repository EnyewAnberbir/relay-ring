//! Integration test for `RR-0756` (empty).
//! Extended: Journal OTLP bridge export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0756_journal_otlp_bridge_expo_extended_empty() {
    assert!(relayring::capabilities::rr_0756_journal_otlp_bridge_expo_extended::evaluate(&[]).is_err(), "RR-0756: empty input must fail for Extended: Journal OTLP bridge export adapter v6");
}
