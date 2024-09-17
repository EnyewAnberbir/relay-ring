//! Integration test for `RR-0936` (empty).
//! Extended: Gate gateway ack replay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0936_gate_gateway_ack_replay_extended_empty() {
    assert!(relayring::capabilities::rr_0936_gate_gateway_ack_replay_extended::evaluate(&[]).is_err(), "RR-0936: empty input must fail for Extended: Gate gateway ack replay extend codec v11");
}
