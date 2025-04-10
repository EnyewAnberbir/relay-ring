//! Integration test for `RR-0449` (stability).
//! Gate gateway ack replay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0449_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc6, 0xc8];
    let full = relayring::capabilities::rr_0449_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0449: bulk Gate gateway ack replay optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0449_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0449: stable prefix");
        assert!(partial.consumed <= end, "RR-0449: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0449: full prefix should match bulk checksum");
        }
    }
}
