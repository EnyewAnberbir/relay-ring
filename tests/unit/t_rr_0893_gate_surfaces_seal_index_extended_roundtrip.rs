//! Integration test for `RR-0893` (roundtrip).
//! Extended: Gate surfaces seal index refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0893_gate_surfaces_seal_index_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x86, 0x88];
    let a = relayring::capabilities::rr_0893_gate_surfaces_seal_index_extended::evaluate(fixture).expect("RR-0893 first pass");
    let b = relayring::capabilities::rr_0893_gate_surfaces_seal_index_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
