//! Integration test for `RR-0043` (stability).
//! Ring buffer core refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0043_ring_buffer_core_refacto_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let full = relayring::capabilities::rr_0043_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0043: bulk Ring buffer core refactor mutator v18");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0043_ring_buffer_core_refacto::evaluate(&fixture[..end]).expect("RR-0043: stable prefix");
        assert!(partial.consumed <= end, "RR-0043: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0043: full prefix should match bulk checksum");
        }
    }
}
