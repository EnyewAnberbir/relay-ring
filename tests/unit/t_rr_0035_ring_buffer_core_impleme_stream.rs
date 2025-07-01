//! Integration test for `RR-0035` (stream).
//! Ring buffer core implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0035_ring_buffer_core_impleme_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x28];
    let direct = relayring::capabilities::rr_0035_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0035: direct Ring buffer core implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0035_ring_buffer_core_impleme::evaluate(&copied).expect("RR-0035: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0035: stream path must consume input");
}
