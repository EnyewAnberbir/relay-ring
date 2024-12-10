//! Integration test for `RR-0553` (roundtrip).
//! Extended: Ring buffer core refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0553_ring_buffer_core_refacto_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x32];
    let a = relayring::capabilities::rr_0553_ring_buffer_core_refacto_extended::evaluate(fixture).expect("RR-0553 first pass");
    let b = relayring::capabilities::rr_0553_ring_buffer_core_refacto_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
