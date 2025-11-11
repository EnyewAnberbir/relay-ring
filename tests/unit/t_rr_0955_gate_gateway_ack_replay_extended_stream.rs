//! Integration test for `RR-0955` (stream).
//! Extended: Gate gateway ack replay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0955_gate_gateway_ack_replay_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0xc6];
    let direct = relayring::capabilities::rr_0955_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0955: direct Extended: Gate gateway ack replay implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0955_gate_gateway_ack_replay_extended::evaluate(&copied).expect("RR-0955: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0955: stream path must consume input");
}
