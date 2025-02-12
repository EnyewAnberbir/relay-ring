//! Integration test for `RR-0046` (stability).
//! Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0046_ring_buffer_core_extend_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let full = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(fixture).expect("RR-0046: bulk Ring buffer core extend codec v21");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(&fixture[..end]).expect("RR-0046: stable prefix");
        assert!(partial.consumed <= end, "RR-0046: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0046: full prefix should match bulk checksum");
        }
    }
}
