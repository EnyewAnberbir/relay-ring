//! Integration test for `RR-0932` (stability).
//! Extended: Gate gateway ack replay integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0932_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xad, 0xaf];
    let full = relayring::capabilities::rr_0932_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0932: bulk Extended: Gate gateway ack replay integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0932_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0932: stable prefix");
        assert!(partial.consumed <= end, "RR-0932: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0932: full prefix should match bulk checksum");
        }
    }
}
