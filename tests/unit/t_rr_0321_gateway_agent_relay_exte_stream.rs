//! Integration test for `RR-0321` (stream).
//! Gateway agent relay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0321_gateway_agent_relay_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let direct = relayring::capabilities::rr_0321_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0321: direct Gateway agent relay extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0321_gateway_agent_relay_exte::evaluate(&copied).expect("RR-0321: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0321: stream path must consume input");
}
