//! Integration test for `RR-0914` (basic).
//! Extended: Gate compact checksum export benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0914_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9b, 0x9d];
    let first = relayring::capabilities::rr_0914_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0914: Extended: Gate compact checksum export benchmark reporter v19");
    let second = relayring::capabilities::rr_0914_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0914: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0914: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0914: stats visits every byte");
}
