//! Integration test for `RR-0557` (stability).
//! Extended: Ring buffer core harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0557_ring_buffer_core_harden_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let full = relayring::capabilities::rr_0557_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0557: bulk Extended: Ring buffer core harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0557_ring_buffer_core_harden_extended::evaluate(&fixture[..end]).expect("RR-0557: stable prefix");
        assert!(partial.consumed <= end, "RR-0557: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0557: full prefix should match bulk checksum");
        }
    }
}
