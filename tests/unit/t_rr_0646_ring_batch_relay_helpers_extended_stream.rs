//! Integration test for `RR-0646` (stream).
//! Extended: Ring batch relay helpers extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0646_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8d, 0x8f];
    let direct = relayring::capabilities::rr_0646_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0646: direct Extended: Ring batch relay helpers extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0646_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0646: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0646: stream path must consume input");
}
