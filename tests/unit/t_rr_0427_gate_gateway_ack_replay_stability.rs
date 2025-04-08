//! Integration test for `RR-0427` (stability).
//! Gate gateway ack replay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0427_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb0, 0xb2];
    let full = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0427: bulk Gate gateway ack replay harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0427_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0427: stable prefix");
        assert!(partial.consumed <= end, "RR-0427: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0427: full prefix should match bulk checksum");
        }
    }
}
