//! Integration test for `RR-0146` (empty).
//! Ring batch relay helpers extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0146_ring_batch_relay_helpers_empty() {
    assert!(relayring::capabilities::rr_0146_ring_batch_relay_helpers::evaluate(&[]).is_err(), "RR-0146: empty input must fail for Ring batch relay helpers extend codec v1");
}
