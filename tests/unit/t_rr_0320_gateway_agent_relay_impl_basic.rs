//! Integration test for `RR-0320` (basic).
//! Gateway agent relay implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0320_gateway_agent_relay_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let first = relayring::capabilities::rr_0320_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0320: Gateway agent relay implement pipeline v20");
    let second = relayring::capabilities::rr_0320_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0320: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0320: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0320: stats visits every byte");
}
