//! Integration test for `RR-0310` (stream).
//! Gateway agent relay implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0310_gateway_agent_relay_impl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let direct = relayring::capabilities::rr_0310_gateway_agent_relay_impl::evaluate(fixture).expect("RR-0310: direct Gateway agent relay implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0310_gateway_agent_relay_impl::evaluate(&copied).expect("RR-0310: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0310: stream path must consume input");
}
