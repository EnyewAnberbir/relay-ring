//! Integration test for `RR-0531` (roundtrip).
//! Extended: Ring buffer core export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0531_ring_buffer_core_export_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let a = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0531 first pass");
    let b = relayring::capabilities::rr_0531_ring_buffer_core_export_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
