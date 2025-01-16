//! Integration test for `RR-0835` (roundtrip).
//! Extended: Gateway agent relay validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0835_gateway_agent_relay_vali_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x4e];
    let a = relayring::capabilities::rr_0835_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0835 first pass");
    let b = relayring::capabilities::rr_0835_gateway_agent_relay_vali_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
