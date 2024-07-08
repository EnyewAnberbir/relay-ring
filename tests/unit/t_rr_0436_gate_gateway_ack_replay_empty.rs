//! Integration test for `RR-0436` (empty).
//! Gate gateway ack replay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0436_gate_gateway_ack_replay_empty() {
    assert!(relayring::capabilities::rr_0436_gate_gateway_ack_replay::evaluate(&[]).is_err(), "RR-0436: empty input must fail for Gate gateway ack replay extend codec v11");
}
