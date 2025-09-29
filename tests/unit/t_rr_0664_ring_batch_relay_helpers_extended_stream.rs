//! Integration test for `RR-0664` (stream).
//! Extended: Ring batch relay helpers benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0664_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9f, 0xa1];
    let direct = relayring::capabilities::rr_0664_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0664: direct Extended: Ring batch relay helpers benchmark reporter v19");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0664_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0664: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0664: stream path must consume input");
}
