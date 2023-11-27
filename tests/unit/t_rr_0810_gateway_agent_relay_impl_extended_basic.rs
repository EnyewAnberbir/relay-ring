//! Integration test for `RR-0810` (basic).
//! Extended: Gateway agent relay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0810_gateway_agent_relay_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let first = relayring::capabilities::rr_0810_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0810: Extended: Gateway agent relay implement pipeline v10");
    let second = relayring::capabilities::rr_0810_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0810: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0810: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0810: scanner should emit domain hints");
}
