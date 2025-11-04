//! Integration test for `RR-0909` (stream).
//! Extended: Gate compact checksum export optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0909_gate_compact_checksum_ex_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x96, 0x98];
    let direct = relayring::capabilities::rr_0909_gate_compact_checksum_ex_extended::evaluate(fixture).expect("RR-0909: direct Extended: Gate compact checksum export optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0909_gate_compact_checksum_ex_extended::evaluate(&copied).expect("RR-0909: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0909: stream path must consume input");
}
