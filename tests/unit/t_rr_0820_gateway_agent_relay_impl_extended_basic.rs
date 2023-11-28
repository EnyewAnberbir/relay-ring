//! Integration test for `RR-0820` (basic).
//! Extended: Gateway agent relay implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0820_gateway_agent_relay_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let first = relayring::capabilities::rr_0820_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0820: Extended: Gateway agent relay implement pipeline v20");
    let second = relayring::capabilities::rr_0820_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0820: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0820: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0820: window consumes the whole buffer");
}
