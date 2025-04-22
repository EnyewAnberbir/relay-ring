//! Integration test for `RR-0529` (stability).
//! Extended: Ring buffer core optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0529_ring_buffer_core_optimiz_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let full = relayring::capabilities::rr_0529_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0529: bulk Extended: Ring buffer core optimize registry v4");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0529_ring_buffer_core_optimiz_extended::evaluate(&fixture[..end]).expect("RR-0529: stable prefix");
        assert!(partial.consumed <= end, "RR-0529: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0529: full prefix should match bulk checksum");
        }
    }
}
