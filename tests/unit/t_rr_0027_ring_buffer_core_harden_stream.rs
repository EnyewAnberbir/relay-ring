//! Integration test for `RR-0027` (stream).
//! Ring buffer core harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0027_ring_buffer_core_harden_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let direct = relayring::capabilities::rr_0027_ring_buffer_core_harden::evaluate(fixture).expect("RR-0027: direct Ring buffer core harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0027_ring_buffer_core_harden::evaluate(&copied).expect("RR-0027: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0027: stream path must consume input");
}
