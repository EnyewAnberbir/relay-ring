//! Integration test for `RR-0553` (stream).
//! Extended: Ring buffer core refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0553_ring_buffer_core_refacto_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let direct = relayring::capabilities::rr_0553_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0553: direct Extended: Ring buffer core refactor mutator v28");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0553_ring_buffer_core_refacto_extended::evaluate(&copied).expect("RR-0553: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0553: stream path must consume input");
}
