//! Integration test for `RR-0034` (roundtrip).
//! Ring buffer core benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0034_ring_buffer_core_benchma_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let a = relayring::capabilities::rr_0034_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0034 first pass");
    let b = relayring::capabilities::rr_0034_ring_buffer_core_benchma::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
