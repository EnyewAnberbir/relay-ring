//! Integration test for `RR-0830` (stream).
//! Extended: Gateway agent relay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0830_gateway_agent_relay_impl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let direct = relayring::capabilities::rr_0830_gateway_agent_relay_impl_extended::evaluate(fixture).expect("RR-0830: direct Extended: Gateway agent relay implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0830_gateway_agent_relay_impl_extended::evaluate(&copied).expect("RR-0830: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0830: stream path must consume input");
}
