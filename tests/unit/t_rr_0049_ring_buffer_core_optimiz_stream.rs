//! Integration test for `RR-0049` (stream).
//! Ring buffer core optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0049_ring_buffer_core_optimiz_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let direct = relayring::capabilities::rr_0049_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0049: direct Ring buffer core optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0049_ring_buffer_core_optimiz::evaluate(&copied).expect("RR-0049: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0049: stream path must consume input");
}
