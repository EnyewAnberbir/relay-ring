//! Integration test for `RR-0367` (roundtrip).
//! Gate surfaces seal index harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0367_gate_surfaces_seal_index_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0x76];
    let a = relayring::capabilities::rr_0367_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0367 first pass");
    let b = relayring::capabilities::rr_0367_gate_surfaces_seal_index::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
