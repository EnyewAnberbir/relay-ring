//! Integration test for `RR-0308` (stream).
//! Gateway agent relay refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0308_gateway_agent_relay_refa_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let direct = relayring::capabilities::rr_0308_gateway_agent_relay_refa::evaluate(fixture).expect("RR-0308: direct Gateway agent relay refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0308_gateway_agent_relay_refa::evaluate(&copied).expect("RR-0308: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0308: stream path must consume input");
}
