//! Integration test for `RR-0533` (stability).
//! Extended: Ring buffer core refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0533_ring_buffer_core_refacto_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let full = relayring::capabilities::rr_0533_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0533: bulk Extended: Ring buffer core refactor mutator v8");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0533_ring_buffer_core_refacto_extended::evaluate(&fixture[..end]).expect("RR-0533: stable prefix");
        assert!(partial.consumed <= end, "RR-0533: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0533: full prefix should match bulk checksum");
        }
    }
}
