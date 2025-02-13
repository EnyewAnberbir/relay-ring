//! Integration test for `RR-0057` (stability).
//! Ring buffer core harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0057_ring_buffer_core_harden_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let full = relayring::capabilities::rr_0057_ring_buffer_core_harden::evaluate(fixture).expect("RR-0057: bulk Ring buffer core harden index v32");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0057_ring_buffer_core_harden::evaluate(&fixture[..end]).expect("RR-0057: stable prefix");
        assert!(partial.consumed <= end, "RR-0057: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0057: full prefix should match bulk checksum");
        }
    }
}
