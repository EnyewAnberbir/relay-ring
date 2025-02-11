//! Integration test for `RR-0032` (stability).
//! Ring buffer core integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0032_ring_buffer_core_integra_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let full = relayring::capabilities::rr_0032_ring_buffer_core_integra::evaluate(fixture).expect("RR-0032: bulk Ring buffer core integrate validator v7");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0032_ring_buffer_core_integra::evaluate(&fixture[..end]).expect("RR-0032: stable prefix");
        assert!(partial.consumed <= end, "RR-0032: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0032: full prefix should match bulk checksum");
        }
    }
}
