//! Integration test for `RR-0899` (basic).
//! Extended: Gate compact checksum export optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0899_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8c, 0x8e];
    let first = relayring::capabilities::rr_0899_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0899: Extended: Gate compact checksum export optimize registry v4");
    let second = relayring::capabilities::rr_0899_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0899: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0899: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0899: scanner should emit domain hints");
}
