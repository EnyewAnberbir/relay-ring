//! Integration test for `RR-0555` (stability).
//! Extended: Ring buffer core implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0555_ring_buffer_core_impleme_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let full = relayring::capabilities::rr_0555_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0555: bulk Extended: Ring buffer core implement pipeline v30");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0555_ring_buffer_core_impleme_extended::evaluate(&fixture[..end]).expect("RR-0555: stable prefix");
        assert!(partial.consumed <= end, "RR-0555: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0555: full prefix should match bulk checksum");
        }
    }
}
