//! Integration test for `RR-0558` (stability).
//! Extended: Ring buffer core wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0558_ring_buffer_core_wire_pl_extended_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let full = relayring::capabilities::rr_0558_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0558: bulk Extended: Ring buffer core wire planner v33");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0558_ring_buffer_core_wire_pl_extended::evaluate(&fixture[..end]).expect("RR-0558: stable prefix");
        assert!(partial.consumed <= end, "RR-0558: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0558: full prefix should match bulk checksum");
        }
    }
}
