//! Integration test for `RR-0528` (stability).
//! Extended: Ring buffer core wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0528_ring_buffer_core_wire_pl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x19];
    let full = relayring::capabilities::rr_0528_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0528: bulk Extended: Ring buffer core wire planner v3");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0528_ring_buffer_core_wire_pl_extended::evaluate(&fixture[..end]).expect("RR-0528: stable prefix");
        assert!(partial.consumed <= end, "RR-0528: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0528: full prefix should match bulk checksum");
        }
    }
}
