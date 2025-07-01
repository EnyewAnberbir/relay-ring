//! Integration test for `RR-0037` (stream).
//! Ring buffer core harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0037_ring_buffer_core_harden_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let direct = relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(fixture).expect("RR-0037: direct Ring buffer core harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(&copied).expect("RR-0037: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0037: stream path must consume input");
}
