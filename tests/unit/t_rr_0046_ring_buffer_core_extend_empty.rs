//! Integration test for `RR-0046` (empty).
//! Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0046_ring_buffer_core_extend_empty() {
    assert!(relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(&[]).is_err(), "RR-0046: empty input must fail for Ring buffer core extend codec v21");
}
