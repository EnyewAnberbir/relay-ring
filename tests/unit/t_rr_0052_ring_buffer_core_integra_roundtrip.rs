//! Integration test for `RR-0052` (roundtrip).
//! Ring buffer core integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0052_ring_buffer_core_integra_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x37, 0x39];
    let a = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(fixture).expect("RR-0052 first pass");
    let b = relayring::capabilities::rr_0052_ring_buffer_core_integra::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
