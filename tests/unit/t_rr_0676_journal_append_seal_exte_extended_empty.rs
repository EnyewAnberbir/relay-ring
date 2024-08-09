//! Integration test for `RR-0676` (empty).
//! Extended: Journal append seal extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0676_journal_append_seal_exte_extended_empty() {
    assert!(relayring::capabilities::rr_0676_journal_append_seal_exte_extended::evaluate(&[]).is_err(), "RR-0676: empty input must fail for Extended: Journal append seal extend codec v1");
}
