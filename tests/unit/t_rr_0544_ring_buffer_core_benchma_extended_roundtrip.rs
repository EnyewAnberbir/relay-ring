//! Integration test for `RR-0544` (roundtrip).
//! Extended: Ring buffer core benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0544_ring_buffer_core_benchma_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let a = relayring::capabilities::rr_0544_ring_buffer_core_benchma_extended::evaluate(fixture).expect("RR-0544 first pass");
    let b = relayring::capabilities::rr_0544_ring_buffer_core_benchma_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
