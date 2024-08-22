//! Integration test for `RR-0755` (empty).
//! Extended: Journal OTLP bridge validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0755_journal_otlp_bridge_vali_extended_empty() {
    assert!(relayring::capabilities::rr_0755_journal_otlp_bridge_vali_extended::evaluate(&[]).is_err(), "RR-0755: empty input must fail for Extended: Journal OTLP bridge validate resolver v5");
}
