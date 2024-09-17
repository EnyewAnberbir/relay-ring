//! Integration test for `RR-0938` (empty).
//! Extended: Gate gateway ack replay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0938_gate_gateway_ack_replay_extended_empty() {
    assert!(relayring::capabilities::rr_0938_gate_gateway_ack_replay_extended::evaluate(&[]).is_err(), "RR-0938: empty input must fail for Extended: Gate gateway ack replay wire planner v13");
}
