//! Integration test for `RR-0898` (roundtrip).
//! Extended: Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0898_gate_compact_checksum_ex_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x8d];
    let a = relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0898 first pass");
    let b = relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
