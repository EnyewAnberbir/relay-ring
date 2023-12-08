//! Integration test for `RR-0910` (basic).
//! Extended: Gate compact checksum export validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0910_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x97, 0x99];
    let first = relayring::capabilities::rr_0910_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0910: Extended: Gate compact checksum export validate resolver v15");
    let second = relayring::capabilities::rr_0910_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0910: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0910: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0910: window consumes the whole buffer");
}
