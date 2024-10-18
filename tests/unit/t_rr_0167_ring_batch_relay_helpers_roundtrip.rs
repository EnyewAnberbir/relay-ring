//! Integration test for `RR-0167` (roundtrip).
//! Ring batch relay helpers harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0167_ring_batch_relay_helpers_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0xac];
    let a = relayring::capabilities::rr_0167_ring_batch_relay_helpers::evaluate(fixture).expect("RR-0167 first pass");
    let b = relayring::capabilities::rr_0167_ring_batch_relay_helpers::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
