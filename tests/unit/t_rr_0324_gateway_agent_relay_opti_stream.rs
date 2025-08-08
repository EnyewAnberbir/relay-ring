//! Integration test for `RR-0324` (stream).
//! Gateway agent relay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0324_gateway_agent_relay_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let direct = relayring::capabilities::rr_0324_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0324: direct Gateway agent relay optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0324_gateway_agent_relay_opti::evaluate(&copied).expect("RR-0324: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0324: stream path must consume input");
}
