//! Integration test for `RR-0054` (stream).
//! Ring buffer core benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0054_ring_buffer_core_benchma_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39, 0x3b];
    let direct = relayring::capabilities::rr_0054_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0054: direct Ring buffer core benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0054_ring_buffer_core_benchma::evaluate(&copied).expect("RR-0054: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0054: stream path must consume input");
}
