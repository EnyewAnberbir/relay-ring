//! Integration test for `RR-0318` (stream).
//! Gateway agent relay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0318_gateway_agent_relay_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let direct = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0318: direct Gateway agent relay refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0318_gateway_agent_relay_refa::evaluate(&copied).expect("RR-0318: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0318: stream path must consume input");
}
