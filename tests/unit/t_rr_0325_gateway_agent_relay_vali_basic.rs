//! Integration test for `RR-0325` (basic).
//! Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0325_gateway_agent_relay_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let first = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0325: Gateway agent relay validate resolver v25");
    let second = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0325: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0325: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0325: window consumes the whole buffer");
}
