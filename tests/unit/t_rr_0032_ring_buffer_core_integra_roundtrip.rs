//! Integration test for `RR-0032` (roundtrip).
//! Ring buffer core integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0032_ring_buffer_core_integra_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let a = relayring::capabilities::rr_0032_ring_buffer_core_integra::evaluate(fixture).expect("RR-0032 first pass");
    let b = relayring::capabilities::rr_0032_ring_buffer_core_integra::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
