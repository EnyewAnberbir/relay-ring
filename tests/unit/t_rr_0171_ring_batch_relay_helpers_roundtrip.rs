//! Integration test for `RR-0171` (roundtrip).
//! Ring batch relay helpers export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0171_ring_batch_relay_helpers_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xae, 0xb0];
    let a = relayring::capabilities::rr_0171_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0171 first pass");
    let b = relayring::capabilities::rr_0171_ring_batch_relay_helpers::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
