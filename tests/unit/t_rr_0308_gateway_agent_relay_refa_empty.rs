//! Integration test for `RR-0308` (empty).
//! Gateway agent relay refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0308_gateway_agent_relay_refa_empty() {
    assert!(relayring::capabilities::rr_0308_gateway_agent_relay_refa::evaluate(&[]).is_err(), "RR-0308: empty input must fail for Gateway agent relay refactor mutator v8");
}
