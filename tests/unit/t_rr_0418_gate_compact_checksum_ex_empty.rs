//! Integration test for `RR-0418` (empty).
//! Gate compact checksum export wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0418_gate_compact_checksum_ex_empty() {
    assert!(relayring::capabilities::rr_0418_gate_compact_checksum_ex::evaluate(&[]).is_err(), "RR-0418: empty input must fail for Gate compact checksum export wire planner v23");
}
