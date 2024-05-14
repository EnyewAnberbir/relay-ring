//! Integration test for `RR-0038` (empty).
//! Ring buffer core wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0038_ring_buffer_core_wire_pl_empty() {
    assert!(relayring::capabilities::rr_0038_ring_buffer_core_wire_pl::evaluate(&[]).is_err(), "RR-0038: empty input must fail for Ring buffer core wire planner v13");
}
