//! Integration test for `RR-0329` (stream).
//! Gateway agent relay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0329_gateway_agent_relay_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x50];
    let direct = relayring::capabilities::rr_0329_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0329: direct Gateway agent relay benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0329_gateway_agent_relay_benc::evaluate(&copied).expect("RR-0329: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0329: stream path must consume input");
}
