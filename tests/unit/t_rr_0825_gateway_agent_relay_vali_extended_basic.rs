//! Integration test for `RR-0825` (basic).
//! Extended: Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0825_gateway_agent_relay_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x44];
    let first = relayring::capabilities::rr_0825_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0825: Extended: Gateway agent relay validate resolver v25");
    let second = relayring::capabilities::rr_0825_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0825: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0825: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0825: stats visits every byte");
}
