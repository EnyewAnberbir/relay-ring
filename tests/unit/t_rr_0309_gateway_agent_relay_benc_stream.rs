//! Integration test for `RR-0309` (stream).
//! Gateway agent relay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0309_gateway_agent_relay_benc_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let direct = relayring::capabilities::rr_0309_gateway_agent_relay_benc::evaluate(fixture).expect("RR-0309: direct Gateway agent relay benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0309_gateway_agent_relay_benc::evaluate(&copied).expect("RR-0309: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0309: stream path must consume input");
}
