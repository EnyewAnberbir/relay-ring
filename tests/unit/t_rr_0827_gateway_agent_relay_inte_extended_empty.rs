//! Integration test for `RR-0827` (empty).
//! Extended: Gateway agent relay integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0827_gateway_agent_relay_inte_extended_empty() {
    assert!(relayring::capabilities::rr_0827_gateway_agent_relay_inte_extended::evaluate(&[]).is_err(), "RR-0827: empty input must fail for Extended: Gateway agent relay integrate validator v27");
}
