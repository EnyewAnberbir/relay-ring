//! Integration test for `RR-0555` (stream).
//! Extended: Ring buffer core implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0555_ring_buffer_core_impleme_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let direct = relayring::capabilities::rr_0555_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0555: direct Extended: Ring buffer core implement pipeline v30");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0555_ring_buffer_core_impleme_extended::evaluate(&copied).expect("RR-0555: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0555: stream path must consume input");
}
