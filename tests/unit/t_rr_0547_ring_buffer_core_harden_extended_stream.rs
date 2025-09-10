//! Integration test for `RR-0547` (stream).
//! Extended: Ring buffer core harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0547_ring_buffer_core_harden_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let direct = relayring::capabilities::rr_0547_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0547: direct Extended: Ring buffer core harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0547_ring_buffer_core_harden_extended::evaluate(&copied).expect("RR-0547: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0547: stream path must consume input");
}
