//! Integration test for `RR-0940` (stability).
//! Extended: Gate gateway ack replay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0940_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb5, 0xb7];
    let full = relayring::capabilities::rr_0940_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0940: bulk Extended: Gate gateway ack replay validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0940_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0940: stable prefix");
        assert!(partial.consumed <= end, "RR-0940: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0940: full prefix should match bulk checksum");
        }
    }
}
