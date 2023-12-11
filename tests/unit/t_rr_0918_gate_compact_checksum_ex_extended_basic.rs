//! Integration test for `RR-0918` (basic).
//! Extended: Gate compact checksum export wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0918_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9f, 0xa1];
    let first = relayring::capabilities::rr_0918_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0918: Extended: Gate compact checksum export wire planner v23");
    let second = relayring::capabilities::rr_0918_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0918: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0918: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0918: stats visits every byte");
}
