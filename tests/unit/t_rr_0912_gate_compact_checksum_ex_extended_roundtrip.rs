//! Integration test for `RR-0912` (roundtrip).
//! Extended: Gate compact checksum export integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0912_gate_compact_checksum_ex_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x9b];
    let a = relayring::capabilities::rr_0912_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0912 first pass");
    let b = relayring::capabilities::rr_0912_gate_compact_checksum_ex_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
