//! Integration test for `RR-0934` (stability).
//! Extended: Gate gateway ack replay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0934_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaf, 0xb1];
    let full = relayring::capabilities::rr_0934_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0934: bulk Extended: Gate gateway ack replay benchmark reporter v9");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0934_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0934: stable prefix");
        assert!(partial.consumed <= end, "RR-0934: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0934: full prefix should match bulk checksum");
        }
    }
}
