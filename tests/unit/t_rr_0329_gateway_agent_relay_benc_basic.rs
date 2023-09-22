//! Integration test for `RR-0329` (basic).
//! Gateway agent relay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0329_gateway_agent_relay_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let first = relayring::capabilities::rr_0329_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0329: Gateway agent relay benchmark reporter v29");
    let second = relayring::capabilities::rr_0329_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0329: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0329: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0329: scanner should emit domain hints");
}
