//! Integration test for `RR-0554` (stability).
//! Extended: Ring buffer core benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0554_ring_buffer_core_benchma_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let full = relayring::capabilities::rr_0554_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0554: bulk Extended: Ring buffer core benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0554_ring_buffer_core_benchma_extended::evaluate(&fixture[..end]).expect("RR-0554: stable prefix");
        assert!(partial.consumed <= end, "RR-0554: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0554: full prefix should match bulk checksum");
        }
    }
}
