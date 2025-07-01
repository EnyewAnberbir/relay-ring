//! Integration test for `RR-0031` (stream).
//! Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0031_ring_buffer_core_export_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let direct = relayring::capabilities::rr_0031_ring_buffer_core_export::evaluate(fixture).expect("RR-0031: direct Ring buffer core export adapter v6");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0031_ring_buffer_core_export::evaluate(&copied).expect("RR-0031: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0031: stream path must consume input");
}
