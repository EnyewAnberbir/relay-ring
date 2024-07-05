//! Integration test for `RR-0427` (empty).
//! Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0427_gate_gateway_ack_replay_empty() {
    assert!(relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(&[]).is_err(), "RR-0427: empty input must fail for Gate gateway ack replay harden index v2");
}
