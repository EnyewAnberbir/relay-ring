//! Integration test for `RR-0531` (stability).
//! Extended: Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0531_ring_buffer_core_export_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let full = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0531: bulk Extended: Ring buffer core export adapter v6");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(&fixture[..end]).expect("RR-0531: stable prefix");
        assert!(partial.consumed <= end, "RR-0531: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0531: full prefix should match bulk checksum");
        }
    }
}
