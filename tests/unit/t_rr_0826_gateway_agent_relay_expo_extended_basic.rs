//! Integration test for `RR-0826` (basic).
//! Extended: Gateway agent relay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0826_gateway_agent_relay_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let first = relayring::capabilities::rr_0826_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0826: Extended: Gateway agent relay export adapter v26");
    let second = relayring::capabilities::rr_0826_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0826: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0826: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0826: window consumes the whole buffer");
}
