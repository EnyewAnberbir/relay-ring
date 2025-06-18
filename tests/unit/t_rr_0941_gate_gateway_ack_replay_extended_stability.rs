//! Integration test for `RR-0941` (stability).
//! Extended: Gate gateway ack replay export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0941_gate_gateway_ack_replay_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb6, 0xb8];
    let full = relayring::capabilities::rr_0941_gate_gateway_ack_replay_extended::evaluate(fixture).expect("RR-0941: bulk Extended: Gate gateway ack replay export adapter v16");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0941_gate_gateway_ack_replay_extended::evaluate(&fixture[..end]).expect("RR-0941: stable prefix");
        assert!(partial.consumed <= end, "RR-0941: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0941: full prefix should match bulk checksum");
        }
    }
}
