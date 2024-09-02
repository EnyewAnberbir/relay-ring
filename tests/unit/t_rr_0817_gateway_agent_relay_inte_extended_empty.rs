//! Integration test for `RR-0817` (empty).
//! Extended: Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0817_gateway_agent_relay_inte_extended_empty() {
    assert!(relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(&[]).is_err(), "RR-0817: empty input must fail for Extended: Gateway agent relay integrate validator v17");
}
