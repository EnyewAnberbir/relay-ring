//! Integration test for `RR-0051` (stream).
//! Ring buffer core export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0051_ring_buffer_core_export_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let direct = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(fixture).expect("RR-0051: direct Ring buffer core export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(&copied).expect("RR-0051: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0051: stream path must consume input");
}
