//! Integration test for `RR-0939` (stability).
//! Extended: Gate gateway ack replay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0939_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let full = relayring::capabilities::rr_0939_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0939: bulk Extended: Gate gateway ack replay optimize registry v14");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0939_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0939: stable prefix");
        assert!(partial.consumed <= end, "RR-0939: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0939: full prefix should match bulk checksum");
        }
    }
}
