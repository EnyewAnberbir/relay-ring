//! Integration test for `RR-0546` (roundtrip).
//! Extended: Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0546_ring_buffer_core_extend_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let a = relayring::capabilities::rr_0546_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0546 first pass");
    let b = relayring::capabilities::rr_0546_ring_buffer_core_extend_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
