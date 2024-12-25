//! Integration test for `RR-0666` (roundtrip).
//! Extended: Ring batch relay helpers extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0666_ring_batch_relay_helpers_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xa3];
    let a = relayring::capabilities::rr_0666_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0666 first pass");
    let b = relayring::capabilities::rr_0666_ring_batch_relay_helpers_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
