//! Integration test for `RR-0817` (basic).
//! Extended: Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0817_gateway_agent_relay_inte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let first = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0817: Extended: Gateway agent relay integrate validator v17");
    let second = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0817: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0817: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0817: stats visits every byte");
}
