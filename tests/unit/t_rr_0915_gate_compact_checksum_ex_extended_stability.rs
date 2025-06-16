//! Integration test for `RR-0915` (stability).
//! Extended: Gate compact checksum export implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0915_gate_compact_checksum_ex_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9c, 0x9e];
    let full = relayring::capabilities::rr_0915_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0915: bulk Extended: Gate compact checksum export implement pipeline v20");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0915_gate_compact_checksum_ex_extended::evaluate(&fixture[..end]).expect("RR-0915: stable prefix");
        assert!(partial.consumed <= end, "RR-0915: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0915: full prefix should match bulk checksum");
        }
    }
}
