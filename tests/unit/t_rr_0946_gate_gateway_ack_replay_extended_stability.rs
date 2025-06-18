//! Integration test for `RR-0946` (stability).
//! Extended: Gate gateway ack replay extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0946_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbb, 0xbd];
    let full = relayring::capabilities::rr_0946_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0946: bulk Extended: Gate gateway ack replay extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0946_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0946: stable prefix");
        assert!(partial.consumed <= end, "RR-0946: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0946: full prefix should match bulk checksum");
        }
    }
}
