//! Integration test for `RR-0055` (roundtrip).
//! Ring buffer core implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0055_ring_buffer_core_impleme_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let a = relayring::capabilities::rr_0055_ring_buffer_core_impleme::evaluate(fixture).expect("RR-0055 first pass");
    let b = relayring::capabilities::rr_0055_ring_buffer_core_impleme::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
