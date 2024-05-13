//! Integration test for `RR-0027` (empty).
//! Ring buffer core harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0027_ring_buffer_core_harden_empty() {
    assert!(relayring::capabilities::rr_0027_ring_buffer_core_harden::evaluate(&[]).is_err(), "RR-0027: empty input must fail for Ring buffer core harden index v2");
}
