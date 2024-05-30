//! Integration test for `RR-0178` (empty).
//! Journal append seal wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0178_journal_append_seal_wire_empty() {
    assert!(relayring::capabilities::rr_0178_journal_append_seal_wire::evaluate(&[]).is_err(), "RR-0178: empty input must fail for Journal append seal wire planner v3");
}
