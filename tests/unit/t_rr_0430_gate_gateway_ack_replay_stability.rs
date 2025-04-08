//! Integration test for `RR-0430` (stability).
//! Gate gateway ack replay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0430_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb3, 0xb5];
    let full = relayring::capabilities::rr_0430_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0430: bulk Gate gateway ack replay validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0430_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0430: stable prefix");
        assert!(partial.consumed <= end, "RR-0430: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0430: full prefix should match bulk checksum");
        }
    }
}
