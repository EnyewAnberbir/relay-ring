//! Integration test for `RR-0544` (stream).
//! Extended: Ring buffer core benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0544_ring_buffer_core_benchma_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let direct = relayring::capabilities::rr_0544_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0544: direct Extended: Ring buffer core benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0544_ring_buffer_core_benchma_extended::evaluate(&copied).expect("RR-0544: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0544: stream path must consume input");
}
