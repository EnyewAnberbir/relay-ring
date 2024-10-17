//! Integration test for `RR-0162` (roundtrip).
//! Ring batch relay helpers integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0162_ring_batch_relay_helpers_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let a = relayring::capabilities::rr_0162_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0162 first pass");
    let b = relayring::capabilities::rr_0162_ring_batch_relay_helpers::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
