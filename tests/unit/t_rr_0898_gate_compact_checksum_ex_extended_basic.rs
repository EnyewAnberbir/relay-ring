//! Integration test for `RR-0898` (basic).
//! Extended: Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0898_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8b, 0x8d];
    let first = relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0898: Extended: Gate compact checksum export wire planner v3");
    let second = relayring::capabilities::rr_0898_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0898: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0898: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0898: stats visits every byte");
}
