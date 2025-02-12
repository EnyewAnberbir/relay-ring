//! Integration test for `RR-0042` (stability).
//! Ring buffer core integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0042_ring_buffer_core_integra_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x2f];
    let full = relayring::capabilities::rr_0042_ring_buffer_core_integra::evaluate(fixture).expect("RR-0042: bulk Ring buffer core integrate validator v17");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0042_ring_buffer_core_integra::evaluate(&fixture[..end]).expect("RR-0042: stable prefix");
        assert!(partial.consumed <= end, "RR-0042: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0042: full prefix should match bulk checksum");
        }
    }
}
