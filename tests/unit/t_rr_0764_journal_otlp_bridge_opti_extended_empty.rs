//! Integration test for `RR-0764` (empty).
//! Extended: Journal OTLP bridge optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0764_journal_otlp_bridge_opti_extended_empty() {
    assert!(relayring::capabilities::rr_0764_journal_otlp_bridge_opti_extended::evaluate(&[]).is_err(), "RR-0764: empty input must fail for Extended: Journal OTLP bridge optimize registry v14");
}
