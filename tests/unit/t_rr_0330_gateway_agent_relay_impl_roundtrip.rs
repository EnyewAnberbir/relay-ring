//! Integration test for `RR-0330` (roundtrip).
//! Gateway agent relay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0330_gateway_agent_relay_impl_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4f, 0x51];
    let a = relayring::capabilities::rr_0330_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0330 first pass");
    let b = relayring::capabilities::rr_0330_gateway_agent_relay_impl::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
