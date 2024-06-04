//! Integration test for `RR-0206` (empty).
//! Journal append seal extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0206_journal_append_seal_exte_empty() {
    assert!(relayring::capabilities::rr_0206_journal_append_seal_exte::evaluate(&[]).is_err(), "RR-0206: empty input must fail for Journal append seal extend codec v31");
}
