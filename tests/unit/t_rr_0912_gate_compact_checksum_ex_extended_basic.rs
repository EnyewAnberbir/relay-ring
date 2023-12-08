//! Integration test for `RR-0912` (basic).
//! Extended: Gate compact checksum export integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0912_gate_compact_checksum_ex_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0x9b];
    let first = relayring::capabilities::rr_0912_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0912: Extended: Gate compact checksum export integrate validator v17");
    let second = relayring::capabilities::rr_0912_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0912: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0912: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0912: window consumes the whole buffer");
}
