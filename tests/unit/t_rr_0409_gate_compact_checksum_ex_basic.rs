//! Integration test for `RR-0409` (basic).
//! Gate compact checksum export optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0409_gate_compact_checksum_ex_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x9e, 0xa0];
    let first = relayring::capabilities::rr_0409_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0409: Gate compact checksum export optimize registry v14");
    let second = relayring::capabilities::rr_0409_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0409: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0409: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0409: window consumes the whole buffer");
}
