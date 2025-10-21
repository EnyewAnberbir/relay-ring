//! Integration test for `RR-0810` (stream).
//! Extended: Gateway agent relay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0810_gateway_agent_relay_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let direct = relayring::capabilities::rr_0810_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0810: direct Extended: Gateway agent relay implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0810_gateway_agent_relay_impl_extended::evaluate(&copied).expect("RR-0810: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0810: stream path must consume input");
}
