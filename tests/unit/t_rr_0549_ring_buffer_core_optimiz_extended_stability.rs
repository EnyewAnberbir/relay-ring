//! Integration test for `RR-0549` (stability).
//! Extended: Ring buffer core optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0549_ring_buffer_core_optimiz_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let full = relayring::capabilities::rr_0549_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0549: bulk Extended: Ring buffer core optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0549_ring_buffer_core_optimiz_extended::evaluate(&fixture[..end]).expect("RR-0549: stable prefix");
        assert!(partial.consumed <= end, "RR-0549: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0549: full prefix should match bulk checksum");
        }
    }
}
