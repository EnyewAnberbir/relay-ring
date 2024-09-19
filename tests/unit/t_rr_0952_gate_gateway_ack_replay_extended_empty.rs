//! Integration test for `RR-0952` (empty).
//! Extended: Gate gateway ack replay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0952_gate_gateway_ack_replay_extended_empty() {
    assert!(relayring::capabilities::rr_0952_gate_gateway_ack_replay_extended::evaluate(&[]).is_err(), "RR-0952: empty input must fail for Extended: Gate gateway ack replay integrate validator v27");
}
