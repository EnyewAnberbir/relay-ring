//! Integration test for `RR-0762` (empty).
//! Extended: Journal OTLP bridge harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0762_journal_otlp_bridge_hard_extended_empty() {
    assert!(relayring::capabilities::rr_0762_journal_otlp_bridge_hard_extended::evaluate(&[]).is_err(), "RR-0762: empty input must fail for Extended: Journal OTLP bridge harden index v12");
}
