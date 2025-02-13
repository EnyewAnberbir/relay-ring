//! Integration test for `RR-0052` (stability).
//! Ring buffer core integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0052_ring_buffer_core_integra_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let full = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(fixture).expect("RR-0052: bulk Ring buffer core integrate validator v27");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(&fixture[..end]).expect("RR-0052: stable prefix");
        assert!(partial.consumed <= end, "RR-0052: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0052: full prefix should match bulk checksum");
        }
    }
}
