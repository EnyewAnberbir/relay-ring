//! Integration test for `RR-0438` (stability).
//! Gate gateway ack replay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0438_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let full = relayring::capabilities::rr_0438_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0438: bulk Gate gateway ack replay wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0438_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0438: stable prefix");
        assert!(partial.consumed <= end, "RR-0438: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0438: full prefix should match bulk checksum");
        }
    }
}
