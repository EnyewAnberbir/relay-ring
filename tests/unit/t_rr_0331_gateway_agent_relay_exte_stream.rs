//! Integration test for `RR-0331` (stream).
//! Gateway agent relay extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0331_gateway_agent_relay_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x52];
    let direct = relayring::capabilities::rr_0331_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0331: direct Gateway agent relay extend codec v31");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0331_gateway_agent_relay_exte::evaluate(&copied).expect("RR-0331: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0331: stream path must consume input");
}
