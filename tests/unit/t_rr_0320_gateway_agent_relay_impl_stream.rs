//! Integration test for `RR-0320` (stream).
//! Gateway agent relay implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0320_gateway_agent_relay_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let direct = relayring::capabilities::rr_0320_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0320: direct Gateway agent relay implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0320_gateway_agent_relay_impl::evaluate(&copied).expect("RR-0320: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0320: stream path must consume input");
}
