//! Integration test for `RR-0648` (stream).
//! Extended: Ring batch relay helpers wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0648_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8f, 0x91];
    let direct = relayring::capabilities::rr_0648_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0648: direct Extended: Ring batch relay helpers wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0648_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0648: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0648: stream path must consume input");
}
