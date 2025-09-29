//! Integration test for `RR-0660` (stream).
//! Extended: Ring batch relay helpers validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0660_ring_batch_relay_helpers_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9b, 0x9d];
    let direct = relayring::capabilities::rr_0660_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0660: direct Extended: Ring batch relay helpers validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0660_ring_batch_relay_helpers_extended::evaluate(&copied).expect("RR-0660: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0660: stream path must consume input");
}
