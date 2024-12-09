//! Integration test for `RR-0540` (roundtrip).
//! Extended: Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0540_ring_buffer_core_validat_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let a = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0540 first pass");
    let b = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
