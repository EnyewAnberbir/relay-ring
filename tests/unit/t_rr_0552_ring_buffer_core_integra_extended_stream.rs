//! Integration test for `RR-0552` (stream).
//! Extended: Ring buffer core integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0552_ring_buffer_core_integra_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let direct = relayring::capabilities::rr_0552_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0552: direct Extended: Ring buffer core integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0552_ring_buffer_core_integra_extended::evaluate(&copied).expect("RR-0552: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0552: stream path must consume input");
}
