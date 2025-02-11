//! Integration test for `RR-0040` (stability).
//! Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0040_ring_buffer_core_validat_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let full = relayring::capabilities::rr_0040_ring_buffer_core_validat::evaluate(fixture).expect("RR-0040: bulk Ring buffer core validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0040_ring_buffer_core_validat::evaluate(&fixture[..end]).expect("RR-0040: stable prefix");
        assert!(partial.consumed <= end, "RR-0040: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0040: full prefix should match bulk checksum");
        }
    }
}
