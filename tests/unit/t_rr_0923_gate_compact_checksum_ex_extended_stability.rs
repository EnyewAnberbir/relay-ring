//! Integration test for `RR-0923` (stability).
//! Extended: Gate compact checksum export refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0923_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0xa6];
    let full = relayring::capabilities::rr_0923_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0923: bulk Extended: Gate compact checksum export refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0923_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0923: stable prefix");
        assert!(partial.consumed <= end, "RR-0923: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0923: full prefix should match bulk checksum");
        }
    }
}
