//! Integration test for `RR-0053` (stability).
//! Ring buffer core refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0053_ring_buffer_core_refacto_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let full = relayring::capabilities::rr_0053_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0053: bulk Ring buffer core refactor mutator v28");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0053_ring_buffer_core_refacto::evaluate(&fixture[..end]).expect("RR-0053: stable prefix");
        assert!(partial.consumed <= end, "RR-0053: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0053: full prefix should match bulk checksum");
        }
    }
}
