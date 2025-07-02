//! Integration test for `RR-0039` (stream).
//! Ring buffer core optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0039_ring_buffer_core_optimiz_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let direct = relayring::capabilities::rr_0039_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0039: direct Ring buffer core optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0039_ring_buffer_core_optimiz::evaluate(&copied).expect("RR-0039: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0039: stream path must consume input");
}
