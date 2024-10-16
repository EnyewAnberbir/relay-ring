//! Integration test for `RR-0150` (roundtrip).
//! Ring batch relay helpers validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0150_ring_batch_relay_helpers_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x9b];
    let a = relayring::capabilities::rr_0150_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0150 first pass");
    let b = relayring::capabilities::rr_0150_ring_batch_relay_helpers::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
