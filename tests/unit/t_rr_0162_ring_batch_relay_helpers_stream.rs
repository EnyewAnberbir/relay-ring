//! Integration test for `RR-0162` (stream).
//! Ring batch relay helpers integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0162_ring_batch_relay_helpers_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let direct = relayring::capabilities::rr_0162_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0162: direct Ring batch relay helpers integrate validator v17");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0162_ring_batch_relay_helpers::evaluate(&copied).expect("RR-0162: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0162: stream path must consume input");
}
