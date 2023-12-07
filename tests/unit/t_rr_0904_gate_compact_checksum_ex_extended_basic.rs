//! Integration test for `RR-0904` (basic).
//! Extended: Gate compact checksum export benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0904_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x91, 0x93];
    let first = relayring::capabilities::rr_0904_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0904: Extended: Gate compact checksum export benchmark reporter v9");
    let second = relayring::capabilities::rr_0904_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0904: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0904: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0904: stats visits every byte");
}
