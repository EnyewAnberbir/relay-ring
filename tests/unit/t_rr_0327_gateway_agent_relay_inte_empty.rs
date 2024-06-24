//! Integration test for `RR-0327` (empty).
//! Gateway agent relay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0327_gateway_agent_relay_inte_empty() {
    assert!(relayring::capabilities::rr_0327_gateway_agent_relay_inte::evaluate(&[]).is_err(), "RR-0327: empty input must fail for Gateway agent relay integrate validator v27");
}
