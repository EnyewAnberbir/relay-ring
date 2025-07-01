//! Integration test for `RR-0036` (stream).
//! Ring buffer core extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0036_ring_buffer_core_extend_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let direct = relayring::capabilities::rr_0036_ring_buffer_core_extend::evaluate(fixture).expect("RR-0036: direct Ring buffer core extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0036_ring_buffer_core_extend::evaluate(&copied).expect("RR-0036: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0036: stream path must consume input");
}
