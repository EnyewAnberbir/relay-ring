//! Integration test for `RR-0051` (stability).
//! Ring buffer core export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0051_ring_buffer_core_export_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let full = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(fixture).expect("RR-0051: bulk Ring buffer core export adapter v26");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(&fixture[..end]).expect("RR-0051: stable prefix");
        assert!(partial.consumed <= end, "RR-0051: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0051: full prefix should match bulk checksum");
        }
    }
}
