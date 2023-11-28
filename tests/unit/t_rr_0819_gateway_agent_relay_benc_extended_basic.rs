//! Integration test for `RR-0819` (basic).
//! Extended: Gateway agent relay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0819_gateway_agent_relay_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let first = relayring::capabilities::rr_0819_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0819: Extended: Gateway agent relay benchmark reporter v19");
    let second = relayring::capabilities::rr_0819_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0819: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0819: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0819: window consumes the whole buffer");
}
