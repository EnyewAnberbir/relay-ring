//! Integration test for `RR-0048` (stability).
//! Ring buffer core wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0048_ring_buffer_core_wire_pl_stability() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let full = relayring::capabilities::rr_0048_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0048: bulk Ring buffer core wire planner v23");
    for end in (fixture.len() / 2)..=fixture.len() {
        let partial = relayring::capabilities::rr_0048_ring_buffer_core_wire_pl::evaluate(&fixture[..end]).expect("RR-0048: stable prefix");
        assert!(partial.consumed <= end, "RR-0048: consumed must not exceed prefix length");
        if end == fixture.len() {
            assert_eq!(partial.checksum, full.checksum, "RR-0048: full prefix should match bulk checksum");
        }
    }
}
