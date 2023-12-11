//! Integration test for `RR-0917` (basic).
//! Extended: Gate compact checksum export harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0917_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9e, 0xa0];
    let first = relayring::capabilities::rr_0917_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0917: Extended: Gate compact checksum export harden index v22");
    let second = relayring::capabilities::rr_0917_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0917: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0917: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0917: scanner should emit domain hints");
}
