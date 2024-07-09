//! Integration test for `RR-0451` (empty).
//! Gate gateway ack replay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0451_gate_gateway_ack_replay_empty() {
    assert!(relayring::capabilities::rr_0451_gate_gateway_ack_replay::evaluate(&[]).is_err(), "RR-0451: empty input must fail for Gate gateway ack replay export adapter v26");
}
