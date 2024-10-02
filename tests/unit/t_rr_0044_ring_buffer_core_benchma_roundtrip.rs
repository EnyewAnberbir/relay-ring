//! Integration test for `RR-0044` (roundtrip).
//! Ring buffer core benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0044_ring_buffer_core_benchma_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let a = relayring::capabilities::rr_0044_ring_buffer_core_benchma::evaluate(fixture).expect("RR-0044 first pass");
    let b = relayring::capabilities::rr_0044_ring_buffer_core_benchma::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
