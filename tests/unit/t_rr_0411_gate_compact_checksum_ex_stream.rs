//! Integration test for `RR-0411` (stream).
//! Gate compact checksum export export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0411_gate_compact_checksum_ex_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0, 0xa2];
    let direct = relayring::capabilities::rr_0411_gate_compact_checksum_ex::evaluate(fixture).expect("RR-0411: direct Gate compact checksum export export adapter v16");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0411_gate_compact_checksum_ex::evaluate(&copied).expect("RR-0411: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0411: stream path must consume input");
}
