//! Integration test for `RR-0026` (empty).
//! Ring buffer core extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0026_ring_buffer_core_extend_empty() {
    assert!(relayring::capabilities::rr_0026_ring_buffer_core_extend::evaluate(&[]).is_err(), "RR-0026: empty input must fail for Ring buffer core extend codec v1");
}
