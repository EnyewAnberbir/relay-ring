//! Integration test for `RR-0532` (stream).
//! Extended: Ring buffer core integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0532_ring_buffer_core_integra_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let direct = relayring::capabilities::rr_0532_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0532: direct Extended: Ring buffer core integrate validator v7");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0532_ring_buffer_core_integra_extended::evaluate(&copied).expect("RR-0532: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0532: stream path must consume input");
}
