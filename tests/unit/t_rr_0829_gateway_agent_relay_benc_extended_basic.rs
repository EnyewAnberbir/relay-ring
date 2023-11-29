//! Integration test for `RR-0829` (basic).
//! Extended: Gateway agent relay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0829_gateway_agent_relay_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let first = relayring::capabilities::rr_0829_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0829: Extended: Gateway agent relay benchmark reporter v29");
    let second = relayring::capabilities::rr_0829_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0829: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0829: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0829: window consumes the whole buffer");
}
