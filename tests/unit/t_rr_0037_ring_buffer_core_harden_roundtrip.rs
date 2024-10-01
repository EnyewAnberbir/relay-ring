//! Integration test for `RR-0037` (roundtrip).
//! Ring buffer core harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0037_ring_buffer_core_harden_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let a = relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(fixture).expect("RR-0037 first pass");
    let b = relayring::capabilities::rr_0037_ring_buffer_core_harden::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
