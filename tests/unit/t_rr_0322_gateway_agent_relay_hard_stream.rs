//! Integration test for `RR-0322` (stream).
//! Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0322_gateway_agent_relay_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x47, 0x49];
    let direct = relayring::capabilities::rr_0322_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0322: direct Gateway agent relay harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0322_gateway_agent_relay_hard::evaluate(&copied).expect("RR-0322: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0322: stream path must consume input");
}
