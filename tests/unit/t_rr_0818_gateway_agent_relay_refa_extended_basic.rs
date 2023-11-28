//! Integration test for `RR-0818` (basic).
//! Extended: Gateway agent relay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0818_gateway_agent_relay_refa_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let first = relayring::capabilities::rr_0818_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0818: Extended: Gateway agent relay refactor mutator v18");
    let second = relayring::capabilities::rr_0818_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0818: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0818: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0818: stats visits every byte");
}
