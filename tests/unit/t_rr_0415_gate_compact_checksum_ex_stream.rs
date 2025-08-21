//! Integration test for `RR-0415` (stream).
//! Gate compact checksum export implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0415_gate_compact_checksum_ex_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa4, 0xa6];
    let direct = relayring::capabilities::rr_0415_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0415: direct Gate compact checksum export implement pipeline v20");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0415_gate_compact_checksum_ex::evaluate(&copied).expect("RR-0415: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0415: stream path must consume input");
}
