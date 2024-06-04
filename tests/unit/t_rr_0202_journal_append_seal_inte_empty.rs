//! Integration test for `RR-0202` (empty).
//! Journal append seal integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0202_journal_append_seal_inte_empty() {
    assert!(relayring::capabilities::rr_0202_journal_append_seal_inte::evaluate(&[]).is_err(), "RR-0202: empty input must fail for Journal append seal integrate validator v27");
}
