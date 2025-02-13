//! Integration test for `RR-0054` (stability).
//! Ring buffer core benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0054_ring_buffer_core_benchma_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let full = relayring::capabilities::rr_0054_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0054: bulk Ring buffer core benchmark reporter v29");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0054_ring_buffer_core_benchma::evaluate(&fixture[..end]).expect("RR-0054: stable prefix");
        assert!(partial.consumed <= end, "RR-0054: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0054: full prefix should match bulk checksum");
        }
    }
}
