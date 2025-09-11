//! Integration test for `RR-0559` (stream).
//! Extended: Ring buffer core optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0559_ring_buffer_core_optimiz_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let direct = relayring::capabilities::rr_0559_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0559: direct Extended: Ring buffer core optimize registry v34");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0559_ring_buffer_core_optimiz_extended::evaluate(&copied).expect("RR-0559: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0559: stream path must consume input");
}
