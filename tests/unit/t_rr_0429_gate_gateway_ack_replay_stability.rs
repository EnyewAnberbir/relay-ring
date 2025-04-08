//! Integration test for `RR-0429` (stability).
//! Gate gateway ack replay optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0429_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb2, 0xb4];
    let full = relayring::capabilities::rr_0429_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0429: bulk Gate gateway ack replay optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0429_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0429: stable prefix");
        assert!(partial.consumed <= end, "RR-0429: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0429: full prefix should match bulk checksum");
        }
    }
}
