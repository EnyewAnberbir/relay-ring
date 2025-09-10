//! Integration test for `RR-0549` (stream).
//! Extended: Ring buffer core optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0549_ring_buffer_core_optimiz_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let direct = relayring::capabilities::rr_0549_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0549: direct Extended: Ring buffer core optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0549_ring_buffer_core_optimiz_extended::evaluate(&copied).expect("RR-0549: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0549: stream path must consume input");
}
