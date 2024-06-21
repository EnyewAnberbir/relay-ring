//! Integration test for `RR-0325` (empty).
//! Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0325_gateway_agent_relay_vali_empty() {
    assert!(relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(&[]).is_err(), "RR-0325: empty input must fail for Gateway agent relay validate resolver v25");
}
