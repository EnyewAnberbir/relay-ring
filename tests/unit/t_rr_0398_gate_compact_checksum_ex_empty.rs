//! Integration test for `RR-0398` (empty).
//! Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0398_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0398_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0398: empty input must fail for Gate compact checksum export wire planner v3");
}
