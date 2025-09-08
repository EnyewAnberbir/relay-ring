//! Integration test for `RR-0536` (stream).
//! Extended: Ring buffer core extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0536_ring_buffer_core_extend_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let direct = relayring::capabilities::rr_0536_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0536: direct Extended: Ring buffer core extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0536_ring_buffer_core_extend_extended::evaluate(&copied).expect("RR-0536: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0536: stream path must consume input");
}
