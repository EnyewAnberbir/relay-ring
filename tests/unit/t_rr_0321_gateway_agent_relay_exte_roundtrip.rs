//! Integration test for `RR-0321` (roundtrip).
//! Gateway agent relay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0321_gateway_agent_relay_exte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let a = relayring::capabilities::rr_0321_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0321 first pass");
    let b = relayring::capabilities::rr_0321_gateway_agent_relay_exte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
