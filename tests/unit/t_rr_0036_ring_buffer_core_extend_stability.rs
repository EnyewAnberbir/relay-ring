//! Integration test for `RR-0036` (stability).
//! Ring buffer core extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0036_ring_buffer_core_extend_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let full = relayring::capabilities::rr_0036_ring_buffer_core_extend::evaluate(fixture).expect("RR-0036: bulk Ring buffer core extend codec v11");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0036_ring_buffer_core_extend::evaluate(&fixture[..end]).expect("RR-0036: stable prefix");
        assert!(partial.consumed <= end, "RR-0036: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0036: full prefix should match bulk checksum");
        }
    }
}
