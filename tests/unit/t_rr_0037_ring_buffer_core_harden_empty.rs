//! Integration test for `RR-0037` (empty).
//! Ring buffer core harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0037_ring_buffer_core_harden_empty() {
    assert!(relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(&[]).is_err(), "RR-0037: empty input must fail for Ring buffer core harden index v12");
}
