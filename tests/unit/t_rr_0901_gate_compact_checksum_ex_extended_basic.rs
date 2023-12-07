//! Integration test for `RR-0901` (basic).
//! Extended: Gate compact checksum export export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0901_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8e, 0x90];
    let first = relayring::capabilities::rr_0901_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0901: Extended: Gate compact checksum export export adapter v6");
    let second = relayring::capabilities::rr_0901_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0901: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0901: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0901: scanner should emit domain hints");
}
