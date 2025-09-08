//! Integration test for `RR-0529` (stream).
//! Extended: Ring buffer core optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0529_ring_buffer_core_optimiz_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let direct = relayring::capabilities::rr_0529_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0529: direct Extended: Ring buffer core optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0529_ring_buffer_core_optimiz_extended::evaluate(&copied).expect("RR-0529: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0529: stream path must consume input");
}
