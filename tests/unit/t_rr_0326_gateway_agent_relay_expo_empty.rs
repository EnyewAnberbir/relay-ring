//! Integration test for `RR-0326` (empty).
//! Gateway agent relay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0326_gateway_agent_relay_expo_empty() {
    assert!(relayring::capabilities::rr_0326_gateway_agent_relay_expo::evaluate(&[]).is_err(), "RR-0326: empty input must fail for Gateway agent relay export adapter v26");
}
