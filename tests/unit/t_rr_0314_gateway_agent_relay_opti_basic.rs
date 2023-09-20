//! Integration test for `RR-0314` (basic).
//! Gateway agent relay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0314_gateway_agent_relay_opti_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let first = relayring::capabilities::rr_0314_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0314: Gateway agent relay optimize registry v14");
    let second = relayring::capabilities::rr_0314_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0314: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0314: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0314: window consumes the whole buffer");
}
