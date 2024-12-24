//! Integration test for `RR-0656` (roundtrip).
//! Extended: Ring batch relay helpers extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0656_ring_batch_relay_helpers_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let a = relayring::capabilities::rr_0656_ring_batch_relay_helpers_extended::evaluate(fixture).expect("RR-0656 first pass");
    let b = relayring::capabilities::rr_0656_ring_batch_relay_helpers_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
