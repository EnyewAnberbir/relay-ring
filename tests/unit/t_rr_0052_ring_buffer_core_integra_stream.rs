//! Integration test for `RR-0052` (stream).
//! Ring buffer core integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0052_ring_buffer_core_integra_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let direct = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(fixture).expect("RR-0052: direct Ring buffer core integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(&copied).expect("RR-0052: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0052: stream path must consume input");
}
