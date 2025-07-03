//! Integration test for `RR-0060` (stream).
//! Ring buffer core validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0060_ring_buffer_core_validat_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let direct = relayring::capabilities::rr_0060_ring_buffer_core_validat::evaluate(fixture).expect("RR-0060: direct Ring buffer core validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0060_ring_buffer_core_validat::evaluate(&copied).expect("RR-0060: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0060: stream path must consume input");
}
