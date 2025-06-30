//! Integration test for `RR-0029` (stream).
//! Ring buffer core optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0029_ring_buffer_core_optimiz_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let direct = relayring::capabilities::rr_0029_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0029: direct Ring buffer core optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0029_ring_buffer_core_optimiz::evaluate(&copied).expect("RR-0029: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0029: stream path must consume input");
}
