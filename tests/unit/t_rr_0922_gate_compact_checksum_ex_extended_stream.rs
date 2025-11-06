//! Integration test for `RR-0922` (stream).
//! Extended: Gate compact checksum export integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0922_gate_compact_checksum_ex_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa3, 0xa5];
    let direct = relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0922: direct Extended: Gate compact checksum export integrate validator v27");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0922_gate_compact_checksum_ex_extended::evaluate(&copied).expect("RR-0922: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0922: stream path must consume input");
}
