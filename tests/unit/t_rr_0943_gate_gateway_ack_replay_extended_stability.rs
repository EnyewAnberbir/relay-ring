//! Integration test for `RR-0943` (stability).
//! Extended: Gate gateway ack replay refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0943_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb8, 0xba];
    let full = relayring::capabilities::rr_0943_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0943: bulk Extended: Gate gateway ack replay refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0943_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0943: stable prefix");
        assert!(partial.consumed <= end, "RR-0943: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0943: full prefix should match bulk checksum");
        }
    }
}
