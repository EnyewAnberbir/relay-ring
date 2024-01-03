//! Integration test for `RR-0058` (bounds).
//! Ring buffer core wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0058_ring_buffer_core_wire_pl_bounds() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    for end in 1..=fixture.len() {
        if let Ok(partial) = relayring::capabilities::rr_0058_ring_buffer_core_wire_pl::evaluate(&fixture[..end]) {
            assert!(partial.consumed <= end);
        }
    }
    let full = relayring::capabilities::rr_0058_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0058 full fixture");
    assert!(full.consumed <= fixture.len());
    assert!(full.consumed > 0);
}
