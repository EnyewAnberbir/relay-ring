//! Integration test for `RR-0556` (stream).
//! Extended: Ring buffer core extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0556_ring_buffer_core_extend_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x35];
    let direct = relayring::capabilities::rr_0556_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0556: direct Extended: Ring buffer core extend codec v31");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0556_ring_buffer_core_extend_extended::evaluate(&copied).expect("RR-0556: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0556: stream path must consume input");
}
