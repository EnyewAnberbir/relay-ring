//! Integration test for `RR-0047` (stability).
//! Ring buffer core harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0047_ring_buffer_core_harden_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let full = relayring::capabilities::rr_0047_ring_buffer_core_harden::evaluate(fixture).expect("RR-0047: bulk Ring buffer core harden index v22");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0047_ring_buffer_core_harden::evaluate(&fixture[..end]).expect("RR-0047: stable prefix");
        assert!(partial.consumed <= end, "RR-0047: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0047: full prefix should match bulk checksum");
        }
    }
}
