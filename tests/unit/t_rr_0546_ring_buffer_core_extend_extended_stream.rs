//! Integration test for `RR-0546` (stream).
//! Extended: Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0546_ring_buffer_core_extend_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let direct = relayring::capabilities::rr_0546_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0546: direct Extended: Ring buffer core extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0546_ring_buffer_core_extend_extended::evaluate(&copied).expect("RR-0546: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0546: stream path must consume input");
}
