//! Integration test for `RR-0196` (empty).
//! Journal append seal extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0196_journal_append_seal_exte_empty() {
    assert!(relayring::capabilities::rr_0196_journal_append_seal_exte::evaluate(&[]).is_err(), "RR-0196: empty input must fail for Journal append seal extend codec v21");
}
