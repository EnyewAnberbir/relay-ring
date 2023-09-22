//! Integration test for `RR-0330` (basic).
//! Gateway agent relay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0330_gateway_agent_relay_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0x51];
    let first = relayring::capabilities::rr_0330_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0330: Gateway agent relay implement pipeline v30");
    let second = relayring::capabilities::rr_0330_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0330: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0330: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0330: scanner should emit domain hints");
}
