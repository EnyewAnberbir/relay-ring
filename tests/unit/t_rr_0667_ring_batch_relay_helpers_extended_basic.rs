//! Integration test for `RR-0667` (basic).
//! Extended: Ring batch relay helpers harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0667_ring_batch_relay_helpers_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa2, 0xa4];
    let first = relayring::capabilities::rr_0667_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0667: Extended: Ring batch relay helpers harden index v22");
    let second = relayring::capabilities::rr_0667_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0667: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0667: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0667: scanner should emit domain hints");
}
