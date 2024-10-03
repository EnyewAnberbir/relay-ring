//! Integration test for `RR-0051` (roundtrip).
//! Ring buffer core export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0051_ring_buffer_core_export_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let a = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(fixture).expect("RR-0051 first pass");
    let b = relayring::capabilities::rr_0051_ring_buffer_core_export::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
