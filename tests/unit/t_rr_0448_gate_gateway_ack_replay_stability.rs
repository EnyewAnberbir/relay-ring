//! Integration test for `RR-0448` (stability).
//! Gate gateway ack replay wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0448_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc5, 0xc7];
    let full = relayring::capabilities::rr_0448_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0448: bulk Gate gateway ack replay wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0448_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0448: stable prefix");
        assert!(partial.consumed <= end, "RR-0448: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0448: full prefix should match bulk checksum");
        }
    }
}
