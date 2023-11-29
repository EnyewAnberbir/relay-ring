//! Integration test for `RR-0830` (basic).
//! Extended: Gateway agent relay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0830_gateway_agent_relay_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let first = relayring::capabilities::rr_0830_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0830: Extended: Gateway agent relay implement pipeline v30");
    let second = relayring::capabilities::rr_0830_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0830: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0830: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0830: scanner should emit domain hints");
}
