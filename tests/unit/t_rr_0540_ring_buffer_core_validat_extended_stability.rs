//! Integration test for `RR-0540` (stability).
//! Extended: Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0540_ring_buffer_core_validat_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let full = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0540: bulk Extended: Ring buffer core validate resolver v15");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(&fixture[..end]).expect("RR-0540: stable prefix");
        assert!(partial.consumed <= end, "RR-0540: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0540: full prefix should match bulk checksum");
        }
    }
}
