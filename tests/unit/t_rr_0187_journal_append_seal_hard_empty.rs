//! Integration test for `RR-0187` (empty).
//! Journal append seal harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0187_journal_append_seal_hard_empty() {
    assert!(relayring::capabilities::rr_0187_journal_append_seal_hard::evaluate(&[]).is_err(), "RR-0187: empty input must fail for Journal append seal harden index v12");
}
