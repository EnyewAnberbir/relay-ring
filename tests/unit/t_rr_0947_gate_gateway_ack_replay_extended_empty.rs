//! Integration test for `RR-0947` (empty).
//! Extended: Gate gateway ack replay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0947_gate_gateway_ack_replay_extended_empty() {
    assert!(relayring::capabilities::rr_0947_gate_gateway_ack_replay_extended::evaluate(&[]).is_err(), "RR-0947: empty input must fail for Extended: Gate gateway ack replay harden index v22");
}
