//! Integration test for `RR-0049` (stability).
//! Ring buffer core optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0049_ring_buffer_core_optimiz_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let full = relayring::capabilities::rr_0049_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0049: bulk Ring buffer core optimize registry v24");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0049_ring_buffer_core_optimiz::evaluate(&fixture[..end]).expect("RR-0049: stable prefix");
        assert!(partial.consumed <= end, "RR-0049: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0049: full prefix should match bulk checksum");
        }
    }
}
