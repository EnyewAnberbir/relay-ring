//! Integration test for `RR-0554` (stream).
//! Extended: Ring buffer core benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0554_ring_buffer_core_benchma_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let direct = relayring::capabilities::rr_0554_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0554: direct Extended: Ring buffer core benchmark reporter v29");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0554_ring_buffer_core_benchma_extended::evaluate(&copied).expect("RR-0554: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0554: stream path must consume input");
}
