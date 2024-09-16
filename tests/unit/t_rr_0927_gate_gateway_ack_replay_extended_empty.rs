//! Integration test for `RR-0927` (empty).
//! Extended: Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0927_gate_gateway_ack_replay_extended_empty() {
    assert!(relayring::capabilities::rr_0927_gate_gateway_ack_replay_extended::evaluate(&[]).is_err(), "RR-0927: empty input must fail for Extended: Gate gateway ack replay harden index v2");
}
