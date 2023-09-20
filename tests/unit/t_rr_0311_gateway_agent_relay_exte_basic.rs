//! Integration test for `RR-0311` (basic).
//! Gateway agent relay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0311_gateway_agent_relay_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let first = relayring::capabilities::rr_0311_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0311: Gateway agent relay extend codec v11");
    let second = relayring::capabilities::rr_0311_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0311: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0311: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0311: scanner should emit domain hints");
}
