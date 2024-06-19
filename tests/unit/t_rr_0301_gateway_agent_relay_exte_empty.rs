//! Integration test for `RR-0301` (empty).
//! Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0301_gateway_agent_relay_exte_empty() {
    assert!(relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(&[]).is_err(), "RR-0301: empty input must fail for Gateway agent relay extend codec v1");
}
