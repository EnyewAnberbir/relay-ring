//! Integration test for `RR-0408` (basic).
//! Gate compact checksum export wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0408_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9d, 0x9f];
    let first = relayring::capabilities::rr_0408_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0408: Gate compact checksum export wire planner v13");
    let second = relayring::capabilities::rr_0408_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0408: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0408: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0408: scanner should emit domain hints");
}
