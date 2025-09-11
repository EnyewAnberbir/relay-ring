//! Integration test for `RR-0558` (stream).
//! Extended: Ring buffer core wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0558_ring_buffer_core_wire_pl_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let direct = relayring::capabilities::rr_0558_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0558: direct Extended: Ring buffer core wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0558_ring_buffer_core_wire_pl_extended::evaluate(&copied).expect("RR-0558: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0558: stream path must consume input");
}
