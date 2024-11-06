//! Integration test for `RR-0302` (roundtrip).
//! Gateway agent relay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0302_gateway_agent_relay_hard_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let a = relayring::capabilities::rr_0302_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0302 first pass");
    let b = relayring::capabilities::rr_0302_gateway_agent_relay_hard::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
