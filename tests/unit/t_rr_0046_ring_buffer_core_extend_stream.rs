//! Integration test for `RR-0046` (stream).
//! Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0046_ring_buffer_core_extend_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let direct = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(fixture).expect("RR-0046: direct Ring buffer core extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(&copied).expect("RR-0046: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0046: stream path must consume input");
}
