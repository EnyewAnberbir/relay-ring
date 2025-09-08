//! Integration test for `RR-0535` (stream).
//! Extended: Ring buffer core implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0535_ring_buffer_core_impleme_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let direct = relayring::capabilities::rr_0535_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0535: direct Extended: Ring buffer core implement pipeline v10");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0535_ring_buffer_core_impleme_extended::evaluate(&copied).expect("RR-0535: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0535: stream path must consume input");
}
