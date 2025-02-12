//! Integration test for `RR-0050` (stability).
//! Ring buffer core validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0050_ring_buffer_core_validat_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let full = relayring::capabilities::rr_0050_ring_buffer_core_validat::evaluate(fixture).expect("RR-0050: bulk Ring buffer core validate resolver v25");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0050_ring_buffer_core_validat::evaluate(&fixture[..end]).expect("RR-0050: stable prefix");
        assert!(partial.consumed <= end, "RR-0050: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0050: full prefix should match bulk checksum");
        }
    }
}
