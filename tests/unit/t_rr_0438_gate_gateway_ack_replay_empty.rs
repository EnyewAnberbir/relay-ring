//! Integration test for `RR-0438` (empty).
//! Gate gateway ack replay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0438_gate_gateway_ack_replay_empty() {
    assert!(relayring::capabilities::rr_0438_gate_gateway_ack_replay::evaluate(&[]).is_err(), "RR-0438: empty input must fail for Gate gateway ack replay wire planner v13");
}
