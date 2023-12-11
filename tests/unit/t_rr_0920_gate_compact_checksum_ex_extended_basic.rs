//! Integration test for `RR-0920` (basic).
//! Extended: Gate compact checksum export validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0920_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa1, 0xa3];
    let first = relayring::capabilities::rr_0920_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0920: Extended: Gate compact checksum export validate resolver v25");
    let second = relayring::capabilities::rr_0920_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0920: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0920: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0920: scanner should emit domain hints");
}
