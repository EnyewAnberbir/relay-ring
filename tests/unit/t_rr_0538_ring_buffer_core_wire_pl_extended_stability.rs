//! Integration test for `RR-0538` (stability).
//! Extended: Ring buffer core wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0538_ring_buffer_core_wire_pl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let full = relayring::capabilities::rr_0538_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0538: bulk Extended: Ring buffer core wire planner v13");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0538_ring_buffer_core_wire_pl_extended::evaluate(&fixture[..end]).expect("RR-0538: stable prefix");
        assert!(partial.consumed <= end, "RR-0538: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0538: full prefix should match bulk checksum");
        }
    }
}
