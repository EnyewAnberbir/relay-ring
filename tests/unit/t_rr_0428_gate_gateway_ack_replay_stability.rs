//! Integration test for `RR-0428` (stability).
//! Gate gateway ack replay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0428_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let full = relayring::capabilities::rr_0428_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0428: bulk Gate gateway ack replay wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0428_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0428: stable prefix");
        assert!(partial.consumed <= end, "RR-0428: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0428: full prefix should match bulk checksum");
        }
    }
}
