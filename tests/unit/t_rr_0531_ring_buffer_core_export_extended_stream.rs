//! Integration test for `RR-0531` (stream).
//! Extended: Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0531_ring_buffer_core_export_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let direct = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0531: direct Extended: Ring buffer core export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(&copied).expect("RR-0531: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0531: stream path must consume input");
}
