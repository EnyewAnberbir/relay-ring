//! Integration test for `RR-0828` (basic).
//! Extended: Gateway agent relay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0828_gateway_agent_relay_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let first = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0828: Extended: Gateway agent relay refactor mutator v28");
    let second = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0828: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0828: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0828: scanner should emit domain hints");
}
