//! Integration test for `RR-0326` (stream).
//! Gateway agent relay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0326_gateway_agent_relay_expo_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x4d];
    let direct = relayring::capabilities::rr_0326_gateway_agent_relay_expo::evaluate(fixture).expect("RR-0326: direct Gateway agent relay export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0326_gateway_agent_relay_expo::evaluate(&copied).expect("RR-0326: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0326: stream path must consume input");
}
