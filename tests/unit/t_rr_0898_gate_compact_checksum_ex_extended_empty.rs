//! Integration test for `RR-0898` (empty).
//! Extended: Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0898_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0898: empty input must fail for Extended: Gate compact checksum export wire planner v3");
}
