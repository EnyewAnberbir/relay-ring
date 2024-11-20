//! Integration test for `RR-0400` (roundtrip).
//! Gate compact checksum export validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0400_gate_compact_checksum_ex_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x97];
    let a = relayring::capabilities::rr_0400_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0400 first pass");
    let b = relayring::capabilities::rr_0400_gate_compact_checksum_ex::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
