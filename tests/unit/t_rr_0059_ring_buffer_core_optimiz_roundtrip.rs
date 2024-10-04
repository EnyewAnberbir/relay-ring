//! Integration test for `RR-0059` (roundtrip).
//! Ring buffer core optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0059_ring_buffer_core_optimiz_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40];
    let a = relayring::capabilities::rr_0059_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0059 first pass");
    let b = relayring::capabilities::rr_0059_ring_buffer_core_optimiz::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
