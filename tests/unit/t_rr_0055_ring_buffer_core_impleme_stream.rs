//! Integration test for `RR-0055` (stream).
//! Ring buffer core implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0055_ring_buffer_core_impleme_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let direct = relayring::capabilities::rr_0055_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0055: direct Ring buffer core implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0055_ring_buffer_core_impleme::evaluate(&copied).expect("RR-0055: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0055: stream path must consume input");
}
