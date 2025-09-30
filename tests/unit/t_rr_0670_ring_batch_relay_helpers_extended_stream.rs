//! Integration test for `RR-0670` (stream).
//! Extended: Ring batch relay helpers validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0670_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let direct = relayring::capabilities::rr_0670_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0670: direct Extended: Ring batch relay helpers validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0670_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0670: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0670: stream path must consume input");
}
