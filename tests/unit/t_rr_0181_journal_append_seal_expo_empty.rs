//! Integration test for `RR-0181` (empty).
//! Journal append seal export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0181_journal_append_seal_expo_empty() {
    assert!(relayring::capabilities::rr_0181_journal_append_seal_expo::evaluate(&[]).is_err(), "RR-0181: empty input must fail for Journal append seal export adapter v6");
}
