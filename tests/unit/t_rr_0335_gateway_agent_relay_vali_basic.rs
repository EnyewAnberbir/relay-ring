//! Integration test for `RR-0335` (basic).
//! Gateway agent relay validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0335_gateway_agent_relay_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let first = relayring::capabilities::rr_0335_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0335: Gateway agent relay validate resolver v35");
    let second = relayring::capabilities::rr_0335_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0335: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0335: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0335: scanner should emit domain hints");
}
