//! Integration test for `RR-0947` (stability).
//! Extended: Gate gateway ack replay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0947_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbc, 0xbe];
    let full = relayring::capabilities::rr_0947_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0947: bulk Extended: Gate gateway ack replay harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0947_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0947: stable prefix");
        assert!(partial.consumed <= end, "RR-0947: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0947: full prefix should match bulk checksum");
        }
    }
}
