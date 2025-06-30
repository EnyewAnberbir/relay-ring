//! Integration test for `RR-0026` (stream).
//! Ring buffer core extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0026_ring_buffer_core_extend_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let direct = relayring::capabilities::rr_0026_ring_buffer_core_extend::evaluate(fixture).expect("RR-0026: direct Ring buffer core extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0026_ring_buffer_core_extend::evaluate(&copied).expect("RR-0026: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0026: stream path must consume input");
}
