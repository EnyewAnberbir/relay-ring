//! Integration test for `RR-0387` (roundtrip).
//! Gate surfaces seal index harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0387_gate_surfaces_seal_index_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0x8a];
    let a = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0387 first pass");
    let b = relayring::capabilities::rr_0387_gate_surfaces_seal_index::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
