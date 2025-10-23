//! Integration test for `RR-0828` (stream).
//! Extended: Gateway agent relay refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0828_gateway_agent_relay_refa_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45, 0x47];
    let direct = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(fixture).expect("RR-0828: direct Extended: Gateway agent relay refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0828_gateway_agent_relay_refa_extended::evaluate(&copied).expect("RR-0828: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0828: stream path must consume input");
}
