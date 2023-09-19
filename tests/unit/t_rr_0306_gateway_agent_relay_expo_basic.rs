//! Integration test for `RR-0306` (basic).
//! Gateway agent relay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0306_gateway_agent_relay_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let first = relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0306: Gateway agent relay export adapter v6");
    let second = relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0306: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0306: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0306: scanner should emit domain hints");
}
