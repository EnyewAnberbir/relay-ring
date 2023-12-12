//! Integration test for `RR-0924` (basic).
//! Extended: Gate compact checksum export benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0924_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa5, 0xa7];
    let first = relayring::capabilities::rr_0924_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0924: Extended: Gate compact checksum export benchmark reporter v29");
    let second = relayring::capabilities::rr_0924_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0924: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0924: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0924: stats visits every byte");
}
