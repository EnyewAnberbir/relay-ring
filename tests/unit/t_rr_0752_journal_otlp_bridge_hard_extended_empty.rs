//! Integration test for `RR-0752` (empty).
//! Extended: Journal OTLP bridge harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0752_journal_otlp_bridge_hard_extended_empty() {
    assert!(relayring::capabilities::rr_0752_journal_otlp_bridge_hard_extended::evaluate(&[]).is_err(), "RR-0752: empty input must fail for Extended: Journal OTLP bridge harden index v2");
}
