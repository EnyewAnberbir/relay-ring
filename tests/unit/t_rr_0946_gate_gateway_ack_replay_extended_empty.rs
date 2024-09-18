//! Integration test for `RR-0946` (empty).
//! Extended: Gate gateway ack replay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0946_gate_gateway_ack_replay_extended_empty() {
    assert!(relayring::capabilities::rr_0946_gate_gateway_ack_replay_extended::evaluate(&[]).is_err(), "RR-0946: empty input must fail for Extended: Gate gateway ack replay extend codec v21");
}
