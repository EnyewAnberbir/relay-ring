//! Integration test for `RR-0560` (stream).
//! Extended: Ring buffer core validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0560_ring_buffer_core_validat_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let direct = relayring::capabilities::rr_0560_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0560: direct Extended: Ring buffer core validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0560_ring_buffer_core_validat_extended::evaluate(&copied).expect("RR-0560: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0560: stream path must consume input");
}
