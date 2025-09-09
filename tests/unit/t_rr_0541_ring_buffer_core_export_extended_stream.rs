//! Integration test for `RR-0541` (stream).
//! Extended: Ring buffer core export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0541_ring_buffer_core_export_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let direct = relayring::capabilities::rr_0541_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0541: direct Extended: Ring buffer core export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0541_ring_buffer_core_export_extended::evaluate(&copied).expect("RR-0541: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0541: stream path must consume input");
}
