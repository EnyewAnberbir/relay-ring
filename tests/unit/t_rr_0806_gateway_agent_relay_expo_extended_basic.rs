//! Integration test for `RR-0806` (basic).
//! Extended: Gateway agent relay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0806_gateway_agent_relay_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let first = relayring::capabilities::rr_0806_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0806: Extended: Gateway agent relay export adapter v6");
    let second = relayring::capabilities::rr_0806_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0806: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0806: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0806: scanner should emit domain hints");
}
