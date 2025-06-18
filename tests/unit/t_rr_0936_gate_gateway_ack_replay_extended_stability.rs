//! Integration test for `RR-0936` (stability).
//! Extended: Gate gateway ack replay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0936_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb1, 0xb3];
    let full = relayring::capabilities::rr_0936_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0936: bulk Extended: Gate gateway ack replay extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0936_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0936: stable prefix");
        assert!(partial.consumed <= end, "RR-0936: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0936: full prefix should match bulk checksum");
        }
    }
}
