//! Integration test for `RR-0426` (empty).
//! Gate gateway ack replay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0426_gate_gateway_ack_replay_empty() {
    assert!(relayring::capabilities::rr_0426_gate_gateway_ack_replay::evaluate(&[]).is_err(), "RR-0426: empty input must fail for Gate gateway ack replay extend codec v1");
}
