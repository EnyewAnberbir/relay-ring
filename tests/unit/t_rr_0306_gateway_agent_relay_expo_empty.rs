//! Integration test for `RR-0306` (empty).
//! Gateway agent relay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0306_gateway_agent_relay_expo_empty() {
    assert!(relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(&[]).is_err(), "RR-0306: empty input must fail for Gateway agent relay export adapter v6");
}
