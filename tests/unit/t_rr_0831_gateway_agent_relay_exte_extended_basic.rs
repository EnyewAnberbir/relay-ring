//! Integration test for `RR-0831` (basic).
//! Extended: Gateway agent relay extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0831_gateway_agent_relay_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x4a];
    let first = relayring::capabilities::rr_0831_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0831: Extended: Gateway agent relay extend codec v31");
    let second = relayring::capabilities::rr_0831_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0831: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0831: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0831: window consumes the whole buffer");
}
