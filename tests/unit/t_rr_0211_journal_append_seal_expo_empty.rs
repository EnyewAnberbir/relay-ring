//! Integration test for `RR-0211` (empty).
//! Journal append seal export adapter v36 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0211_journal_append_seal_expo_empty() {
    assert!(relayring::capabilities::rr_0211_journal_append_seal_expo::evaluate(&[]).is_err(), "RR-0211: empty input must fail for Journal append seal export adapter v36");
}
