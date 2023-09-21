//! Integration test for `RR-0316` (basic).
//! Gateway agent relay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0316_gateway_agent_relay_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let first = relayring::capabilities::rr_0316_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0316: Gateway agent relay export adapter v16");
    let second = relayring::capabilities::rr_0316_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0316: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0316: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0316: stats visits every byte");
}
