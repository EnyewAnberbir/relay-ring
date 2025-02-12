//! Integration test for `RR-0045` (stability).
//! Ring buffer core implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0045_ring_buffer_core_impleme_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let full = relayring::capabilities::rr_0045_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0045: bulk Ring buffer core implement pipeline v20");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0045_ring_buffer_core_impleme::evaluate(&fixture[..end]).expect("RR-0045: stable prefix");
        assert!(partial.consumed <= end, "RR-0045: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0045: full prefix should match bulk checksum");
        }
    }
}
