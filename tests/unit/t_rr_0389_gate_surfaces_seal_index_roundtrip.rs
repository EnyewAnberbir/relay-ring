//! Integration test for `RR-0389` (roundtrip).
//! Gate surfaces seal index optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0389_gate_surfaces_seal_index_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x8c];
    let a = relayring::capabilities::rr_0389_gate_surfaces_seal_index::evaluate(fixture).expect("RR-0389 first pass");
    let b = relayring::capabilities::rr_0389_gate_surfaces_seal_index::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
