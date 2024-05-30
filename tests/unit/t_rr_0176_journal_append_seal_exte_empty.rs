//! Integration test for `RR-0176` (empty).
//! Journal append seal extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0176_journal_append_seal_exte_empty() {
    assert!(relayring::capabilities::rr_0176_journal_append_seal_exte::evaluate(&[]).is_err(), "RR-0176: empty input must fail for Journal append seal extend codec v1");
}
