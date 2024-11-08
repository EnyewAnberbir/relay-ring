//! Integration test for `RR-0320` (roundtrip).
//! Gateway agent relay implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0320_gateway_agent_relay_impl_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let a = relayring::capabilities::rr_0320_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0320 first pass");
    let b = relayring::capabilities::rr_0320_gateway_agent_relay_impl::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
