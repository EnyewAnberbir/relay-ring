//! Integration test for `RR-0534` (stability).
//! Extended: Ring buffer core benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0534_ring_buffer_core_benchma_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let full = relayring::capabilities::rr_0534_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0534: bulk Extended: Ring buffer core benchmark reporter v9");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0534_ring_buffer_core_benchma_extended::evaluate(&fixture[..end]).expect("RR-0534: stable prefix");
        assert!(partial.consumed <= end, "RR-0534: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0534: full prefix should match bulk checksum");
        }
    }
}
