//! Integration test for `RR-0317` (empty).
//! Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0317_gateway_agent_relay_inte_empty() {
    assert!(relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(&[]).is_err(), "RR-0317: empty input must fail for Gateway agent relay integrate validator v17");
}
