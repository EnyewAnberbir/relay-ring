//! Integration test for `RR-0829` (stream).
//! Extended: Gateway agent relay benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0829_gateway_agent_relay_benc_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46, 0x48];
    let direct = relayring::capabilities::rr_0829_gateway_agent_relay_benc_extended::evaluate(fixture).expect("RR-0829: direct Extended: Gateway agent relay benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0829_gateway_agent_relay_benc_extended::evaluate(&copied).expect("RR-0829: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0829: stream path must consume input");
}
