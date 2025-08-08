//! Integration test for `RR-0319` (stream).
//! Gateway agent relay benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0319_gateway_agent_relay_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x44, 0x46];
    let direct = relayring::capabilities::rr_0319_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0319: direct Gateway agent relay benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0319_gateway_agent_relay_benc::evaluate(&copied).expect("RR-0319: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0319: stream path must consume input");
}
