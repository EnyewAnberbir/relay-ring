//! Integration test for `RR-0540` (stream).
//! Extended: Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0540_ring_buffer_core_validat_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let direct = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0540: direct Extended: Ring buffer core validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(&copied).expect("RR-0540: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0540: stream path must consume input");
}
