//! Integration test for `RR-0317` (basic).
//! Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0317_gateway_agent_relay_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x44];
    let first = relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0317: Gateway agent relay integrate validator v17");
    let second = relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0317: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0317: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0317: scanner should emit domain hints");
}
