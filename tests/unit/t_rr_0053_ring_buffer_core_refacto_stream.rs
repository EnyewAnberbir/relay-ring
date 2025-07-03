//! Integration test for `RR-0053` (stream).
//! Ring buffer core refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0053_ring_buffer_core_refacto_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let direct = relayring::capabilities::rr_0053_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0053: direct Ring buffer core refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0053_ring_buffer_core_refacto::evaluate(&copied).expect("RR-0053: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0053: stream path must consume input");
}
