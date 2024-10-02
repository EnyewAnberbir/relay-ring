//! Integration test for `RR-0046` (roundtrip).
//! Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0046_ring_buffer_core_extend_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x31, 0x33];
    let a = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(fixture).expect("RR-0046 first pass");
    let b = relayring::capabilities::rr_0046_ring_buffer_core_extend::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
