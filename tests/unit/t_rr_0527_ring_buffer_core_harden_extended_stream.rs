//! Integration test for `RR-0527` (stream).
//! Extended: Ring buffer core harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0527_ring_buffer_core_harden_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let direct = relayring::capabilities::rr_0527_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0527: direct Extended: Ring buffer core harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0527_ring_buffer_core_harden_extended::evaluate(&copied).expect("RR-0527: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0527: stream path must consume input");
}
