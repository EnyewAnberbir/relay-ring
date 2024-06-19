//! Integration test for `RR-0303` (empty).
//! Gateway agent relay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0303_gateway_agent_relay_wire_empty() {
    assert!(relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(&[]).is_err(), "RR-0303: empty input must fail for Gateway agent relay wire planner v3");
}
