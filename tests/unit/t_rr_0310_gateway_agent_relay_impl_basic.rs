//! Integration test for `RR-0310` (basic).
//! Gateway agent relay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0310_gateway_agent_relay_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let first = relayring::capabilities::rr_0310_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0310: Gateway agent relay implement pipeline v10");
    let second = relayring::capabilities::rr_0310_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0310: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0310: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0310: window consumes the whole buffer");
}
