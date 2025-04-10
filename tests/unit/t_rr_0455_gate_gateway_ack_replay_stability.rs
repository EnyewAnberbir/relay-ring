//! Integration test for `RR-0455` (stability).
//! Gate gateway ack replay implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0455_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xce];
    let full = relayring::capabilities::rr_0455_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0455: bulk Gate gateway ack replay implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0455_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0455: stable prefix");
        assert!(partial.consumed <= end, "RR-0455: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0455: full prefix should match bulk checksum");
        }
    }
}
