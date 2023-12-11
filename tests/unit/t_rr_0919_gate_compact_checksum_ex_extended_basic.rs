//! Integration test for `RR-0919` (basic).
//! Extended: Gate compact checksum export optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0919_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xa2];
    let first = relayring::capabilities::rr_0919_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0919: Extended: Gate compact checksum export optimize registry v24");
    let second = relayring::capabilities::rr_0919_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0919: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0919: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0919: stats visits every byte");
}
