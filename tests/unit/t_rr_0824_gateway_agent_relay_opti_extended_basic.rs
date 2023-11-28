//! Integration test for `RR-0824` (basic).
//! Extended: Gateway agent relay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0824_gateway_agent_relay_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let first = relayring::capabilities::rr_0824_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0824: Extended: Gateway agent relay optimize registry v24");
    let second = relayring::capabilities::rr_0824_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0824: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0824: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0824: stats visits every byte");
}
