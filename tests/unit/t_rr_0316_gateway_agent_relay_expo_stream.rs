//! Integration test for `RR-0316` (stream).
//! Gateway agent relay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0316_gateway_agent_relay_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let direct = relayring::capabilities::rr_0316_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0316: direct Gateway agent relay export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0316_gateway_agent_relay_expo::evaluate(&copied).expect("RR-0316: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0316: stream path must consume input");
}
