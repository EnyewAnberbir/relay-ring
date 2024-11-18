//! Integration test for `RR-0384` (roundtrip).
//! Gate surfaces seal index benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0384_gate_surfaces_seal_index_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x85, 0x87];
    let a = relayring::capabilities::rr_0384_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0384 first pass");
    let b = relayring::capabilities::rr_0384_gate_surfaces_seal_index::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
