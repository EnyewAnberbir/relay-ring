//! Integration test for `RR-0403` (stream).
//! Gate compact checksum export refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0403_gate_compact_checksum_ex_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x9a];
    let direct = relayring::capabilities::rr_0403_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0403: direct Gate compact checksum export refactor mutator v8");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0403_gate_compact_checksum_ex::evaluate(&copied).expect("RR-0403: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0403: stream path must consume input");
}
