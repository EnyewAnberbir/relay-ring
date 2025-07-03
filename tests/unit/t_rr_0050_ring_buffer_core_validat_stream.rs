//! Integration test for `RR-0050` (stream).
//! Ring buffer core validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0050_ring_buffer_core_validat_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let direct = relayring::capabilities::rr_0050_ring_buffer_core_validat::evaluate(fixture).expect("RR-0050: direct Ring buffer core validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0050_ring_buffer_core_validat::evaluate(&copied).expect("RR-0050: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0050: stream path must consume input");
}
