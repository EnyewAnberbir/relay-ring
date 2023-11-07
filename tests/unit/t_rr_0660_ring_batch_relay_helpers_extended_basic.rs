//! Integration test for `RR-0660` (basic).
//! Extended: Ring batch relay helpers validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0660_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9b, 0x9d];
    let first = relayring::capabilities::rr_0660_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0660: Extended: Ring batch relay helpers validate resolver v15");
    let second = relayring::capabilities::rr_0660_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0660: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0660: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0660: stats visits every byte");
}
