//! Integration test for `RR-0312` (stream).
//! Gateway agent relay harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0312_gateway_agent_relay_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let direct = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0312: direct Gateway agent relay harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0312_gateway_agent_relay_hard::evaluate(&copied).expect("RR-0312: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0312: stream path must consume input");
}
