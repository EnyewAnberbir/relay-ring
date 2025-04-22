//! Integration test for `RR-0535` (stability).
//! Extended: Ring buffer core implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0535_ring_buffer_core_impleme_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let full = relayring::capabilities::rr_0535_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0535: bulk Extended: Ring buffer core implement pipeline v10");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0535_ring_buffer_core_impleme_extended::evaluate(&fixture[..end]).expect("RR-0535: stable prefix");
        assert!(partial.consumed <= end, "RR-0535: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0535: full prefix should match bulk checksum");
        }
    }
}
