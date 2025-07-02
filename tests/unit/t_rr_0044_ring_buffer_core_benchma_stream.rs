//! Integration test for `RR-0044` (stream).
//! Ring buffer core benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0044_ring_buffer_core_benchma_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let direct = relayring::capabilities::rr_0044_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0044: direct Ring buffer core benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0044_ring_buffer_core_benchma::evaluate(&copied).expect("RR-0044: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0044: stream path must consume input");
}
