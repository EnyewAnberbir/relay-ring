//! Integration test for `RR-0900` (basic).
//! Extended: Gate compact checksum export validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0900_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8d, 0x8f];
    let first = relayring::capabilities::rr_0900_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0900: Extended: Gate compact checksum export validate resolver v5");
    let second = relayring::capabilities::rr_0900_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0900: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0900: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0900: scanner should emit domain hints");
}
