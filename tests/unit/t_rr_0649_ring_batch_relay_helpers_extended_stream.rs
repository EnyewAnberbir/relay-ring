//! Integration test for `RR-0649` (stream).
//! Extended: Ring batch relay helpers optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0649_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, 0x92];
    let direct = relayring::capabilities::rr_0649_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0649: direct Extended: Ring batch relay helpers optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0649_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0649: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0649: stream path must consume input");
}
