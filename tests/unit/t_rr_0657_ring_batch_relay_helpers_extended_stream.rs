//! Integration test for `RR-0657` (stream).
//! Extended: Ring batch relay helpers harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0657_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let direct = relayring::capabilities::rr_0657_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0657: direct Extended: Ring batch relay helpers harden index v12");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0657_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0657: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0657: stream path must consume input");
}
