//! Integration test for `RR-0541` (roundtrip).
//! Extended: Ring buffer core export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0541_ring_buffer_core_export_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let a = relayring::capabilities::rr_0541_ring_buffer_core_export_extended::evaluate(fixture).expect("RR-0541 first pass");
    let b = relayring::capabilities::rr_0541_ring_buffer_core_export_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
