//! Integration test for `RR-0527` (stability).
//! Extended: Ring buffer core harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0527_ring_buffer_core_harden_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let full = relayring::capabilities::rr_0527_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0527: bulk Extended: Ring buffer core harden index v2");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0527_ring_buffer_core_harden_extended::evaluate(&fixture[..end]).expect("RR-0527: stable prefix");
        assert!(partial.consumed <= end, "RR-0527: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0527: full prefix should match bulk checksum");
        }
    }
}
