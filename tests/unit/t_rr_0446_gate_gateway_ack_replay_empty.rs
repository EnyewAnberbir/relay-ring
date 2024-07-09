//! Integration test for `RR-0446` (empty).
//! Gate gateway ack replay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0446_gate_gateway_ack_replay_empty() {
    assert!(relayring::capabilities::rr_0446_gate_gateway_ack_replay::evaluate(&[]).is_err(), "RR-0446: empty input must fail for Gate gateway ack replay extend codec v21");
}
