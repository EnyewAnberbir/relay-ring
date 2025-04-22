//! Integration test for `RR-0532` (stability).
//! Extended: Ring buffer core integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0532_ring_buffer_core_integra_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let full = relayring::capabilities::rr_0532_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0532: bulk Extended: Ring buffer core integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0532_ring_buffer_core_integra_extended::evaluate(&fixture[..end]).expect("RR-0532: stable prefix");
        assert!(partial.consumed <= end, "RR-0532: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0532: full prefix should match bulk checksum");
        }
    }
}
