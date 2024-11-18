//! Integration test for `RR-0374` (roundtrip).
//! Gate surfaces seal index benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0374_gate_surfaces_seal_index_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b, 0x7d];
    let a = relayring::capabilities::rr_0374_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0374 first pass");
    let b = relayring::capabilities::rr_0374_gate_surfaces_seal_index::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
