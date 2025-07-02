//! Integration test for `RR-0043` (stream).
//! Ring buffer core refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0043_ring_buffer_core_refacto_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let direct = relayring::capabilities::rr_0043_ring_buffer_core_refacto::evaluate(fixture).expect("RR-0043: direct Ring buffer core refactor mutator v18");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0043_ring_buffer_core_refacto::evaluate(&copied).expect("RR-0043: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0043: stream path must consume input");
}
