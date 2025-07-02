//! Integration test for `RR-0042` (stream).
//! Ring buffer core integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0042_ring_buffer_core_integra_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2d, 0x2f];
    let direct = relayring::capabilities::rr_0042_ring_buffer_core_integra::evaluate(fixture).expect("RR-0042: direct Ring buffer core integrate validator v17");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0042_ring_buffer_core_integra::evaluate(&copied).expect("RR-0042: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0042: stream path must consume input");
}
