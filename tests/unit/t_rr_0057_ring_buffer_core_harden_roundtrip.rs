//! Integration test for `RR-0057` (roundtrip).
//! Ring buffer core harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0057_ring_buffer_core_harden_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let a = relayring::capabilities::rr_0057_ring_buffer_core_harden::evaluate(fixture).expect("RR-0057 first pass");
    let b = relayring::capabilities::rr_0057_ring_buffer_core_harden::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
