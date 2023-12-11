//! Integration test for `RR-0922` (basic).
//! Extended: Gate compact checksum export integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0922_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let first = relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0922: Extended: Gate compact checksum export integrate validator v27");
    let second = relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0922: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0922: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0922: stats visits every byte");
}
