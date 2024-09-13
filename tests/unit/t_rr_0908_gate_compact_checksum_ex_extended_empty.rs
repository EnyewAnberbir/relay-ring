//! Integration test for `RR-0908` (empty).
//! Extended: Gate compact checksum export wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0908_gate_compact_checksum_ex_extended_empty() {
    assert!(relayring::capabilities::rr_0908_gate_compact_checksum_ex_extended::evaluate(&[]).is_err(), "RR-0908: empty input must fail for Extended: Gate compact checksum export wire planner v13");
}
