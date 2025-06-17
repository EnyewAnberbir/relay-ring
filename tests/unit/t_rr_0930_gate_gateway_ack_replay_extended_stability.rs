//! Integration test for `RR-0930` (stability).
//! Extended: Gate gateway ack replay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0930_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xab, 0xad];
    let full = relayring::capabilities::rr_0930_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0930: bulk Extended: Gate gateway ack replay validate resolver v5");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0930_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0930: stable prefix");
        assert!(partial.consumed <= end, "RR-0930: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0930: full prefix should match bulk checksum");
        }
    }
}
