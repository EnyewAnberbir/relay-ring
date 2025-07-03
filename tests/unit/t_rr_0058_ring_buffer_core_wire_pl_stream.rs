//! Integration test for `RR-0058` (stream).
//! Ring buffer core wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0058_ring_buffer_core_wire_pl_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3d, 0x3f];
    let direct = relayring::capabilities::rr_0058_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0058: direct Ring buffer core wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0058_ring_buffer_core_wire_pl::evaluate(&copied).expect("RR-0058: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0058: stream path must consume input");
}
