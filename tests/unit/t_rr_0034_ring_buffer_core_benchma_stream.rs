//! Integration test for `RR-0034` (stream).
//! Ring buffer core benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0034_ring_buffer_core_benchma_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let direct = relayring::capabilities::rr_0034_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0034: direct Ring buffer core benchmark reporter v9");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0034_ring_buffer_core_benchma::evaluate(&copied).expect("RR-0034: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0034: stream path must consume input");
}
