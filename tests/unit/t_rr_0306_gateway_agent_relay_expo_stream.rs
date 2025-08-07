//! Integration test for `RR-0306` (stream).
//! Gateway agent relay export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0306_gateway_agent_relay_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let direct = relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0306: direct Gateway agent relay export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0306_gateway_agent_relay_expo::evaluate(&copied).expect("RR-0306: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0306: stream path must consume input");
}
