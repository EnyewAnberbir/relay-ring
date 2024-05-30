//! Integration test for `RR-0172` (empty).
//! Ring batch relay helpers integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0172_ring_batch_relay_helpers_empty() {
    assert!(relayring::capabilities::rr_0172_ring_batch_relay_helpers::evaluate(&[]).is_err(), "RR-0172: empty input must fail for Ring batch relay helpers integrate validator v27");
}
