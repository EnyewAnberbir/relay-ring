//! Integration test for `RR-0434` (stability).
//! Gate gateway ack replay benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0434_gate_gateway_ack_replay_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let full = relayring::capabilities::rr_0434_gate_gateway_ack_replay::evaluate(fixture).expect("RR-0434: bulk Gate gateway ack replay benchmark reporter v9");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0434_gate_gateway_ack_replay::evaluate(&fixture[..end]).expect("RR-0434: stable prefix");
        assert!(partial.consumed <= end, "RR-0434: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0434: full prefix should match bulk checksum");
        }
    }
}
