//! Integration test for `RR-0201` (empty).
//! Journal append seal export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0201_journal_append_seal_expo_empty() {
    assert!(relayring::capabilities::rr_0201_journal_append_seal_expo::evaluate(&[]).is_err(), "RR-0201: empty input must fail for Journal append seal export adapter v26");
}
