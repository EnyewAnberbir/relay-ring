//! Integration test for `RR-0056` (stability).
//! Ring buffer core extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0056_ring_buffer_core_extend_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3b, 0x3d];
    let full = relayring::capabilities::rr_0056_ring_buffer_core_extend::evaluate(fixture).expect("RR-0056: bulk Ring buffer core extend codec v31");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0056_ring_buffer_core_extend::evaluate(&fixture[..end]).expect("RR-0056: stable prefix");
        assert!(partial.consumed <= end, "RR-0056: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0056: full prefix should match bulk checksum");
        }
    }
}
