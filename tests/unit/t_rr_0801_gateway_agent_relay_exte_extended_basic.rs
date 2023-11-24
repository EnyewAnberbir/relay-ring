//! Integration test for `RR-0801` (basic).
//! Extended: Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0801_gateway_agent_relay_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let first = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0801: Extended: Gateway agent relay extend codec v1");
    let second = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0801: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0801: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0801: scanner should emit domain hints");
}
