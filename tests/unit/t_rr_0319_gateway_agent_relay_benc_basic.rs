//! Integration test for `RR-0319` (basic).
//! Gateway agent relay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0319_gateway_agent_relay_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let first = relayring::capabilities::rr_0319_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0319: Gateway agent relay benchmark reporter v19");
    let second = relayring::capabilities::rr_0319_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0319: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0319: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0319: window consumes the whole buffer");
}
