//! Integration test for `RR-0029` (roundtrip).
//! Ring buffer core optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0029_ring_buffer_core_optimiz_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let a = relayring::capabilities::rr_0029_ring_buffer_core_optimiz::evaluate(fixture).expect("RR-0029 first pass");
    let b = relayring::capabilities::rr_0029_ring_buffer_core_optimiz::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
