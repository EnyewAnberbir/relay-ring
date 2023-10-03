//! Integration test for `RR-0398` (basic).
//! Gate compact checksum export wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0398_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x93, 0x95];
    let first = relayring::capabilities::rr_0398_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0398: Gate compact checksum export wire planner v3");
    let second = relayring::capabilities::rr_0398_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0398: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0398: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0398: scanner should emit domain hints");
}
