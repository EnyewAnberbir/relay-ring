//! Integration test for `RR-0166` (stream).
//! Ring batch relay helpers extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0166_ring_batch_relay_helpers_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa9, 0xab];
    let direct = relayring::capabilities::rr_0166_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0166: direct Ring batch relay helpers extend codec v21");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0166_ring_batch_relay_helpers::evaluate(&copied).expect("RR-0166: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0166: stream path must consume input");
}
