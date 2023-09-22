//! Integration test for `RR-0324` (basic).
//! Gateway agent relay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0324_gateway_agent_relay_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let first = relayring::capabilities::rr_0324_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0324: Gateway agent relay optimize registry v24");
    let second = relayring::capabilities::rr_0324_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0324: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0324: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0324: scanner should emit domain hints");
}
