//! Integration test for `RR-0028` (empty).
//! Ring buffer core wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0028_ring_buffer_core_wire_pl_empty() {
    assert!(relayring::capabilities::rr_0028_ring_buffer_core_wire_pl::evaluate(&[]).is_err(), "RR-0028: empty input must fail for Ring buffer core wire planner v3");
}
