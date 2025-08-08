//! Integration test for `RR-0317` (stream).
//! Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0317_gateway_agent_relay_inte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x44];
    let direct = relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(fixture).expect("RR-0317: direct Gateway agent relay integrate validator v17");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0317_gateway_agent_relay_inte::evaluate(&copied).expect("RR-0317: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0317: stream path must consume input");
}
